use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use git2::{Repository, StatusOptions};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use std::{error::Error, io, path::PathBuf, sync::mpsc, thread, time::SystemTime};
use walkdir::WalkDir;

const MASTER_DIR: &str = "..";

use serde::Deserialize;

#[derive(Clone, Default)]
struct GitHubStats {
    stars: u32,
    forks: u32,
    open_issues: u32,
    url: String,
}

#[derive(Deserialize)]
struct GitHubRepoResponse {
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
    html_url: String,
}

fn fetch_github_stats(remote_url: &str) -> Option<GitHubStats> {
    let mut owner_repo = None;

    if remote_url.contains("github.com") {
        if remote_url.starts_with("git@") {
            let parts: Vec<&str> = remote_url.split(':').collect();
            if parts.len() == 2 {
                owner_repo = Some(parts[1].trim_end_matches(".git").to_string());
            }
        } else if remote_url.starts_with("http") {
            if let Some(path) = remote_url.split("github.com/").nth(1) {
                owner_repo = Some(path.trim_end_matches(".git").to_string());
            }
        }
    }

    if let Some(repo_path) = owner_repo {
        let api_url = format!("https://api.github.com/repos/{}", repo_path);
        let mut request = ureq::get(&api_url).header("User-Agent", "Tropical-ProjectManager");

        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            request = request.header("Authorization", &format!("Bearer {}", token));
        }

        if let Ok(mut response) = request.call() {
            if let Ok(data) = response.body_mut().read_json::<GitHubRepoResponse>() {
                return Some(GitHubStats {
                    stars: data.stargazers_count,
                    forks: data.forks_count,
                    open_issues: data.open_issues_count,
                    url: data.html_url,
                });
            }
        }
    }
    None
}

#[derive(Clone)]
enum FileStatusType {
    Modified,
    Untracked,
    Deleted,
}

struct Project {
    name: String,
    path: PathBuf,
    current_branch: String,
    is_dirty: bool,
    untracked: usize,
    modified: usize,
    deleted: usize,
    changed_files: Vec<(String, FileStatusType)>,
    ahead: usize,
    behind: usize,
    last_commit_msg: Option<String>,
    last_commit_time: i64,
    github_stats: Option<GitHubStats>,
}

enum InputMode {
    Normal,
    CreatingProject,
}

struct App {
    projects: Vec<Project>,
    list_state: ListState,
    input_mode: InputMode,
    input_buffer: String,
    is_loading: bool,
    rx: Option<mpsc::Receiver<Vec<Project>>>,
    spinner_tick: u8,
}

impl App {
    fn new() -> App {
        let mut app = App {
            projects: Vec::new(),
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            is_loading: false,
            rx: None,
            spinner_tick: 0,
        };
        app.scan_projects();
        app
    }

    fn scan_projects(&mut self) {
        if self.is_loading {
            return;
        }
        self.is_loading = true;
        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);

        thread::spawn(move || {
            let mut found_projects = Vec::new();
            let walker = WalkDir::new(MASTER_DIR)
                .min_depth(1)
                .max_depth(2)
                .into_iter();

            for entry in walker.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    let git_dir = path.join(".git");
                    if git_dir.exists() {
                        if let Ok(repo) = Repository::open(path) {
                            let name = path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();

                            let mut current_branch = String::from("unknown");
                            let mut last_commit_msg = None;
                            let mut last_commit_time = 0;
                            let mut ahead = 0;
                            let mut behind = 0;

                            if let Ok(head) = repo.head() {
                                if let Some(branch_name) = head.shorthand() {
                                    current_branch = branch_name.to_string();
                                }

                                if let Ok(commit) = head.peel_to_commit() {
                                    last_commit_time = commit.time().seconds();
                                    if let Some(msg) = commit.summary() {
                                        last_commit_msg = Some(msg.to_string());
                                    }
                                }

                                // Try to calculate ahead/behind
                                if head.is_branch() {
                                    if let Some(branch_name) = head.shorthand() {
                                        if let Ok(branch) =
                                            repo.find_branch(branch_name, git2::BranchType::Local)
                                        {
                                            if let Ok(upstream) = branch.upstream() {
                                                if let (Some(local_oid), Some(upstream_oid)) =
                                                    (branch.get().target(), upstream.get().target())
                                                {
                                                    if let Ok((a, b)) = repo
                                                        .graph_ahead_behind(local_oid, upstream_oid)
                                                    {
                                                        ahead = a;
                                                        behind = b;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            let mut untracked = 0;
                            let mut modified = 0;
                            let mut deleted = 0;
                            let mut changed_files = Vec::new();

                            let mut status_opts = StatusOptions::new();
                            status_opts.include_untracked(true);

                            if let Ok(statuses) = repo.statuses(Some(&mut status_opts)) {
                                for s in statuses.iter() {
                                    let status = s.status();
                                    let path_str = s.path().unwrap_or("").to_string();

                                    if status
                                        .intersects(git2::Status::WT_NEW | git2::Status::INDEX_NEW)
                                    {
                                        untracked += 1;
                                        if changed_files.len() < 15 {
                                            changed_files.push((
                                                path_str.clone(),
                                                FileStatusType::Untracked,
                                            ));
                                        }
                                    }
                                    if status.intersects(
                                        git2::Status::WT_MODIFIED
                                            | git2::Status::INDEX_MODIFIED
                                            | git2::Status::WT_RENAMED
                                            | git2::Status::INDEX_RENAMED,
                                    ) {
                                        modified += 1;
                                        if changed_files.len() < 15 {
                                            changed_files
                                                .push((path_str.clone(), FileStatusType::Modified));
                                        }
                                    }
                                    if status.intersects(
                                        git2::Status::WT_DELETED | git2::Status::INDEX_DELETED,
                                    ) {
                                        deleted += 1;
                                        if changed_files.len() < 15 {
                                            changed_files
                                                .push((path_str.clone(), FileStatusType::Deleted));
                                        }
                                    }
                                }
                            }

                            let is_dirty = (untracked + modified + deleted) > 0;

                            let mut github_stats = None;
                            if let Ok(remote) = repo.find_remote("origin") {
                                if let Some(url) = remote.url() {
                                    github_stats = fetch_github_stats(url);
                                }
                            }

                            found_projects.push(Project {
                                name,
                                path: std::fs::canonicalize(path)
                                    .unwrap_or_else(|_| path.to_path_buf()),
                                current_branch,
                                is_dirty,
                                untracked,
                                modified,
                                deleted,
                                changed_files,
                                ahead,
                                behind,
                                last_commit_msg,
                                last_commit_time,
                                github_stats,
                            });
                        }
                    }
                }
            }

            found_projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            let _ = tx.send(found_projects);
        });
    }

    fn next(&mut self) {
        if self.projects.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.projects.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.projects.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.projects.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    // Allows selecting via mouse click on the list
    fn select_index(&mut self, index: usize) {
        if index < self.projects.len() {
            self.list_state.select(Some(index));
        }
    }
}

fn format_time_ago(timestamp: i64) -> String {
    if timestamp == 0 {
        return "Unknown".to_string();
    }
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let diff = now - timestamp;

    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        format!("{} minutes ago", diff / 60)
    } else if diff < 86400 {
        format!("{} hours ago", diff / 3600)
    } else if diff < 2592000 {
        format!("{} days ago", diff / 86400)
    } else if diff < 31536000 {
        format!("{} months ago", diff / 2592000)
    } else {
        format!("{} years ago", diff / 31536000)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Application Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>>
where
    B::Error: 'static,
{
    loop {
        if let Some(rx) = &app.rx {
            if let Ok(projects) = rx.try_recv() {
                app.projects = projects;
                app.is_loading = false;
                app.rx = None;

                if app.list_state.selected().is_none() && !app.projects.is_empty() {
                    app.list_state.select(Some(0));
                }
            }
        }

        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Press {
                        match app.input_mode {
                            InputMode::Normal => match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                                KeyCode::Down | KeyCode::Char('j') => app.next(),
                                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                                KeyCode::Char('r') => {
                                    app.scan_projects();
                                }
                                KeyCode::Char('c') => {
                                    app.input_mode = InputMode::CreatingProject;
                                    app.input_buffer.clear();
                                }
                                _ => {}
                            },
                            InputMode::CreatingProject => match key.code {
                                KeyCode::Enter => {
                                    let new_project_name = app.input_buffer.trim().to_string();
                                    if !new_project_name.is_empty() {
                                        let new_project_path =
                                            PathBuf::from(MASTER_DIR).join(&new_project_name);
                                        let template_dir =
                                            PathBuf::from("./jules_dev_standard/template");

                                        if !new_project_path.exists() && template_dir.exists() {
                                            if std::fs::create_dir_all(&new_project_path).is_ok() {
                                                let mut options = fs_extra::dir::CopyOptions::new();
                                                options.content_only = true;
                                                let _ = fs_extra::dir::copy(
                                                    &template_dir,
                                                    &new_project_path,
                                                    &options,
                                                );
                                                let _ = Repository::init(&new_project_path);
                                            }
                                        }
                                        app.scan_projects();
                                    }
                                    app.input_mode = InputMode::Normal;
                                }
                                KeyCode::Char(c) => {
                                    app.input_buffer.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.input_buffer.pop();
                                }
                                KeyCode::Esc => {
                                    app.input_mode = InputMode::Normal;
                                }
                                _ => {}
                            },
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        event::MouseEventKind::ScrollDown => {
                            if let InputMode::Normal = app.input_mode {
                                app.next();
                            }
                        }
                        event::MouseEventKind::ScrollUp => {
                            if let InputMode::Normal = app.input_mode {
                                app.previous();
                            }
                        }
                        event::MouseEventKind::Down(event::MouseButton::Left) => {
                            if let InputMode::Normal = app.input_mode {
                                let offset = app.list_state.offset();
                                let y_pos = mouse_event.row as usize;
                                // Ignore clicks on the border
                                if y_pos > 0 {
                                    let clicked_index = offset + (y_pos - 1);
                                    app.select_index(clicked_index);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            app.spinner_tick = app.spinner_tick.wrapping_add(1);
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(f.area());

    let items: Vec<ListItem> = app
        .projects
        .iter()
        .map(|p| {
            let prefix = if p.is_dirty { "✗" } else { "✓" };
            let color = if p.is_dirty { Color::Red } else { Color::Green };

            // Add an upstream sync indicator if there's any diff with remote
            let sync_symbol = if p.ahead > 0 && p.behind > 0 {
                " ⇅"
            } else if p.ahead > 0 {
                " ↑"
            } else if p.behind > 0 {
                " ↓"
            } else {
                ""
            };

            let line = Line::from(vec![
                Span::styled(format!("{} ", prefix), Style::default().fg(color)),
                Span::raw(&p.name),
                Span::styled(sync_symbol, Style::default().fg(Color::LightBlue)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let title = if app.is_loading {
        let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame = spinner_frames[(app.spinner_tick as usize / 2) % spinner_frames.len()];
        format!(" Projects {} Scanning... ", frame)
    } else {
        " Projects (q:quit, r:refresh, c:create) ".to_string()
    };

    let projects_block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(if app.is_loading {
                Color::Yellow
            } else {
                Color::White
            }),
        ))
        .borders(Borders::ALL);

    let list = List::new(items)
        .block(projects_block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    let right_chunk = chunks[1];

    if let InputMode::CreatingProject = app.input_mode {
        let input_text = format!("Enter new project name: {}", app.input_buffer);
        let input_paragraph = Paragraph::new(input_text)
            .block(
                Block::default()
                    .title(" Create New Project (Enter:Confirm, Esc:Cancel) ")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input_paragraph, right_chunk);
    } else if let Some(selected_index) = app.list_state.selected() {
        if let Some(project) = app.projects.get(selected_index) {
            // Branch and Sync Status Line
            let mut branch_spans = vec![
                Span::styled("Branch:  ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    &project.current_branch,
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
            ];

            if project.ahead > 0 || project.behind > 0 {
                branch_spans.push(Span::raw(" ("));
                if project.ahead > 0 {
                    branch_spans.push(Span::styled(
                        format!("Ahead: {}", project.ahead),
                        Style::default().fg(Color::Green),
                    ));
                }
                if project.ahead > 0 && project.behind > 0 {
                    branch_spans.push(Span::raw(", "));
                }
                if project.behind > 0 {
                    branch_spans.push(Span::styled(
                        format!("Behind: {}", project.behind),
                        Style::default().fg(Color::Red),
                    ));
                }
                branch_spans.push(Span::raw(")"));
            } else {
                branch_spans.push(Span::styled(
                    " (Synced with remote)",
                    Style::default().fg(Color::DarkGray),
                ));
            }

            // Local Dirty Status Breakdown
            let mut status_spans = vec![
                Span::styled("Status:  ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    if project.is_dirty {
                        "Changes Pending"
                    } else {
                        "Clean"
                    },
                    Style::default().fg(if project.is_dirty {
                        Color::LightYellow
                    } else {
                        Color::LightGreen
                    }),
                ),
            ];

            if project.is_dirty {
                status_spans.push(Span::raw(" ["));
                let mut dirty_parts = Vec::new();
                if project.modified > 0 {
                    dirty_parts.push(Span::styled(
                        format!("~{}", project.modified),
                        Style::default().fg(Color::Yellow),
                    ));
                }
                if project.untracked > 0 {
                    dirty_parts.push(Span::styled(
                        format!("+{}", project.untracked),
                        Style::default().fg(Color::Green),
                    ));
                }
                if project.deleted > 0 {
                    dirty_parts.push(Span::styled(
                        format!("-{}", project.deleted),
                        Style::default().fg(Color::Red),
                    ));
                }

                // Interleave parts with commas
                for (i, part) in dirty_parts.into_iter().enumerate() {
                    if i > 0 {
                        status_spans.push(Span::raw(", "));
                    }
                    status_spans.push(part);
                }
                status_spans.push(Span::raw("]"));
            }

            // Time Since Last Commit
            let time_ago = format_time_ago(project.last_commit_time);

            let mut detail_text = vec![
                Line::from(vec![
                    Span::styled("Project: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        &project.name,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Path:    ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(project.path.to_string_lossy().to_string()),
                ]),
                Line::from(""),
                Line::from(branch_spans),
                Line::from(status_spans),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        "Last Update: ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(time_ago),
                ]),
                Line::from(Span::styled(
                    project
                        .last_commit_msg
                        .as_deref()
                        .unwrap_or("No commits found")
                        .trim(),
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                )),
            ];

            if let Some(gh) = &project.github_stats {
                detail_text.push(Line::from(""));
                detail_text.push(Line::from(vec![
                    Span::styled("GitHub:  ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!(
                            "★ {}   🍴 {}   🐛 {} issues",
                            gh.stars, gh.forks, gh.open_issues
                        ),
                        Style::default().fg(Color::Yellow),
                    ),
                ]));
                detail_text.push(Line::from(vec![
                    Span::styled("URL:     ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        &gh.url,
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                ]));
            }

            if project.is_dirty {
                detail_text.push(Line::from(""));
                detail_text.push(Line::from(Span::styled(
                    "Changed Files:",
                    Style::default().add_modifier(Modifier::BOLD),
                )));
                for (path, status) in &project.changed_files {
                    let (symbol, color) = match status {
                        FileStatusType::Modified => ("~", Color::Yellow),
                        FileStatusType::Untracked => ("+", Color::Green),
                        FileStatusType::Deleted => ("-", Color::Red),
                    };
                    detail_text.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            symbol,
                            Style::default().fg(color).add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" "),
                        Span::styled(path, Style::default().fg(Color::White)),
                    ]));
                }

                let total_changes = project.modified + project.untracked + project.deleted;
                if total_changes > project.changed_files.len() {
                    detail_text.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!(
                                "... and {} more",
                                total_changes - project.changed_files.len()
                            ),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::ITALIC),
                        ),
                    ]));
                }
            } else {
                detail_text.push(Line::from(""));
                detail_text.push(Line::from(Span::styled(
                    "✨ Workspace is clean",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )));
            }

            let detail_paragraph = Paragraph::new(detail_text)
                .block(
                    Block::default()
                        .title(" Project Details ")
                        .borders(Borders::ALL),
                )
                .wrap(Wrap { trim: true });

            f.render_widget(detail_paragraph, right_chunk);
        }
    } else {
        let empty_msg = if app.is_loading {
            "Scanning master directory for projects...".to_string()
        } else if app.projects.is_empty() {
            format!("No projects found in {}. Try 'r' to refresh.", MASTER_DIR)
        } else {
            "Select a project to see details.".to_string()
        };

        let empty_paragraph = Paragraph::new(empty_msg)
            .block(
                Block::default()
                    .title(" Project Details ")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::DarkGray));

        f.render_widget(empty_paragraph, right_chunk);
    }
}
