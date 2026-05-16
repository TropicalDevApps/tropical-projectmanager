// ════════════════════════════════════════════════════════════
//  TropicalUI — TUI Theme Layer
//  Canonical color mapping from TropicalUI design tokens
//  to ratatui Color::Rgb values.
//
//  Token source: TropicalDev-Design/tropical-ui/src/styles/
//    └── index.css      (semantic tokens)
//    └── brand.css      (brand presets)
//    └── tokens.css     (industrial surface ramp)
//
//  Jules Martins / Tropical Media Group
// ════════════════════════════════════════════════════════════

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

// ── BRAND PRESETS ────────────────────────────────────────────
// Maps to @layer brand in TropicalUI CSS
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Brand {
    Tropical, // #4285f4 — default master
    Mango,    // #FFB347
    Ocean,    // #4FA4D8
    Pitahaya, // #FF2400
    Papaya,   // #FF8A65
    Balandra, // #a855f7 (Legacy/Special)
}

impl Default for Brand {
    fn default() -> Self {
        Brand::Tropical
    }
}

// ── MOTION CONSTANTS ─────────────────────────────────────────
// Maps to --ui-dur-* tokens (milliseconds)
pub const DUR_INSTANT_TICKS: u8 = 1;
pub const DUR_FAST_TICKS: u8 = 2;
pub const DUR_BASE_TICKS: u8 = 4;
pub const DUR_SLOW_TICKS: u8 = 6;

// Spinner frames — "orbital" Braille sequence per TUI spec
pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

// Block cursor for active input — --brand-primary bg block
pub const CURSOR_BLOCK: &str = "█";

// Nav orbit active indicator — per TUIComponents spec §4
pub const NAV_ACTIVE_PREFIX: &str = "❯ ";
pub const NAV_IDLE_PREFIX: &str = "  ";

// Tree view characters — Box Drawing Unicode Block (U+2500–U+257F)
pub const TREE_BRANCH: &str = "┣━ ";
pub const TREE_LAST: &str = "┗━ ";
pub const TREE_PIPE: &str = "┃  ";
pub const TREE_DOWN: &str = "▼ ";
pub const TREE_RIGHT: &str = "▶ ";

// Progress density characters — §3 TUI_TELEMETRY_ARRAY
pub const BAR_FULL: char = '█';
pub const BAR_LIGHT: char = '░';

// Status dot characters — .sdot spec
pub const DOT_ON: &str = "●";
pub const DOT_WARN: &str = "●";
pub const DOT_ERR: &str = "●";
pub const DOT_INFO: &str = "●";
pub const DOT_OFF: &str = "○";

// ── THEME STRUCT ─────────────────────────────────────────────
// Mirrors the semantic token slots from tokens.css / index.css
#[derive(Clone, Debug)]
pub struct Theme {
    // Surface ramp — maps to --ui-surface-* (Industrial Slate)
    pub surface_0: Color, // #0a0c10
    pub surface_1: Color, // #0f1218
    pub surface_2: Color, // #14171f
    pub surface_3: Color, // #1a1e26
    pub surface_4: Color, // #21252e
    pub surface_5: Color, // #292e38
    pub surface_6: Color, // #323845

    // Text — maps to --ui-text-*
    pub text_primary: Color,   // rgba(255,255,255,0.92)
    pub text_secondary: Color, // rgba(255,255,255,0.65)
    pub text_tertiary: Color,  // rgba(255,255,255,0.40)
    pub text_disabled: Color,  // rgba(255,255,255,0.20)

    // Accent — maps to --ui-accent-primary-*
    pub primary: Color,       // --brand-primary
    pub primary_light: Color, // --brand-primary-light
    pub primary_bg: Color,    // --brand-primary-bg

    // Accent secondary — maps to --ui-accent-secondary
    pub secondary: Color,

    // Semantic fixed — maps to --ui-success / warning / danger / info
    pub success: Color,       // #34a853
    pub success_light: Color, // #81c995
    pub warning: Color,       // #fbbf24
    pub danger: Color,        // #f28b82
    pub info: Color,          // #60a5fa

    // Border
    pub border_default: Color, // --ui-border-default
    pub border_subtle: Color,  // --ui-border-subtle
}

impl Theme {
    pub fn from_brand(brand: Brand) -> Self {
        // Fixed semantic tokens — brand-agnostic (from index.css)
        let text_primary = Color::Rgb(235, 235, 235);
        let text_secondary = Color::Rgb(166, 166, 166);
        let text_tertiary = Color::Rgb(102, 102, 102);
        let text_disabled = Color::Rgb(51, 51, 51);
        let success = Color::Rgb(52, 168, 83);
        let success_light = Color::Rgb(129, 201, 149);
        let warning = Color::Rgb(251, 191, 36);
        let danger = Color::Rgb(242, 139, 130);
        let info = Color::Rgb(96, 165, 250);
        let border_default = Color::Rgb(33, 37, 46); // --ui-border-default (~surface-4)
        let border_subtle = Color::Rgb(20, 23, 31); // --ui-border-subtle (~surface-2)

        // Base "Industrial Slate" surface ramp (from index.css)
        let s0 = Color::Rgb(10, 12, 16);
        let s1 = Color::Rgb(15, 18, 24);
        let s2 = Color::Rgb(20, 23, 31);
        let s3 = Color::Rgb(26, 30, 38);
        let s4 = Color::Rgb(33, 37, 46);
        let s5 = Color::Rgb(41, 46, 56);
        let s6 = Color::Rgb(50, 56, 69);

        match brand {
            Brand::Tropical => Theme {
                surface_0: s0,
                surface_1: s1,
                surface_2: s2,
                surface_3: s3,
                surface_4: s4,
                surface_5: s5,
                surface_6: s6,
                text_primary,
                text_secondary,
                text_tertiary,
                text_disabled,
                primary: Color::Rgb(66, 133, 244),
                primary_light: Color::Rgb(138, 180, 248),
                primary_bg: Color::Rgb(24, 40, 72),
                secondary: Color::Rgb(197, 169, 245),
                success,
                success_light,
                warning,
                danger,
                info,
                border_default,
                border_subtle,
            },
            Brand::Mango => Theme {
                surface_0: Color::Rgb(17, 16, 13), // Warm variant
                surface_1: Color::Rgb(22, 21, 17),
                surface_2: Color::Rgb(27, 25, 21),
                surface_3: s3,
                surface_4: s4,
                surface_5: s5,
                surface_6: s6,
                text_primary,
                text_secondary,
                text_tertiary,
                text_disabled,
                primary: Color::Rgb(255, 179, 71),
                primary_light: Color::Rgb(255, 204, 128),
                primary_bg: Color::Rgb(255, 243, 224),
                secondary: Color::Rgb(240, 230, 140),
                success,
                success_light,
                warning,
                danger,
                info,
                border_default,
                border_subtle,
            },
            Brand::Ocean => Theme {
                surface_0: s0,
                surface_1: s1,
                surface_2: s2,
                surface_3: s3,
                surface_4: s4,
                surface_5: s5,
                surface_6: s6,
                text_primary,
                text_secondary,
                text_tertiary,
                text_disabled,
                primary: Color::Rgb(79, 164, 216),
                primary_light: Color::Rgb(144, 202, 249),
                primary_bg: Color::Rgb(227, 242, 253),
                secondary: Color::Rgb(128, 203, 196),
                success,
                success_light,
                warning,
                danger,
                info,
                border_default,
                border_subtle,
            },
            Brand::Pitahaya => Theme {
                surface_0: Color::Rgb(18, 12, 13), // Reddish variant
                surface_1: Color::Rgb(24, 17, 18),
                surface_2: Color::Rgb(28, 21, 22),
                surface_3: s3,
                surface_4: s4,
                surface_5: s5,
                surface_6: s6,
                text_primary,
                text_secondary,
                text_tertiary,
                text_disabled,
                primary: Color::Rgb(255, 36, 0),
                primary_light: Color::Rgb(255, 92, 77),
                primary_bg: Color::Rgb(255, 240, 240),
                secondary: Color::Rgb(46, 139, 87),
                success,
                success_light,
                warning,
                danger,
                info,
                border_default,
                border_subtle,
            },
            Brand::Papaya => Theme {
                surface_0: s0,
                surface_1: s1,
                surface_2: s2,
                surface_3: s3,
                surface_4: s4,
                surface_5: s5,
                surface_6: s6,
                text_primary,
                text_secondary,
                text_tertiary,
                text_disabled,
                primary: Color::Rgb(255, 138, 101),
                primary_light: Color::Rgb(255, 171, 145),
                primary_bg: Color::Rgb(251, 233, 231),
                secondary: Color::Rgb(255, 213, 79),
                success,
                success_light,
                warning,
                danger,
                info,
                border_default,
                border_subtle,
            },
            Brand::Balandra => Theme {
                surface_0: Color::Rgb(14, 13, 18), // Purple variant
                surface_1: Color::Rgb(19, 17, 24),
                surface_2: Color::Rgb(24, 21, 29),
                surface_3: s3,
                surface_4: s4,
                surface_5: s5,
                surface_6: s6,
                text_primary,
                text_secondary,
                text_tertiary,
                text_disabled,
                primary: Color::Rgb(168, 85, 247),
                primary_light: Color::Rgb(192, 132, 252),
                primary_bg: Color::Rgb(42, 21, 62),
                secondary: Color::Rgb(225, 29, 72),
                success,
                success_light,
                warning,
                danger,
                info,
                border_default,
                border_subtle,
            },
        }
    }

    // ════════════════════════════════════════════════════════
    //  STYLE HELPERS — semantic Style constructors
    // ════════════════════════════════════════════════════════

    pub fn text(&self) -> Style {
        Style::default().fg(self.text_primary)
    }
    pub fn text_dim(&self) -> Style {
        Style::default().fg(self.text_secondary)
    }
    pub fn text_muted(&self) -> Style {
        Style::default().fg(self.text_tertiary)
    }
    pub fn text_disabled(&self) -> Style {
        Style::default().fg(self.text_disabled)
    }

    pub fn accent(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }
    pub fn accent_light(&self) -> Style {
        Style::default().fg(self.primary_light)
    }
    pub fn accent_secondary(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    pub fn success(&self) -> Style {
        Style::default().fg(self.success_light)
    }
    pub fn warning(&self) -> Style {
        Style::default().fg(self.warning)
    }
    pub fn danger(&self) -> Style {
        Style::default().fg(self.danger)
    }
    pub fn info(&self) -> Style {
        Style::default().fg(self.info)
    }

    /// .tui-status-bar — bg=primary, fg=near-black, bold uppercase
    pub fn status_bar(&self) -> Style {
        Style::default()
            .bg(self.primary)
            .fg(Color::Rgb(5, 5, 5))
            .add_modifier(Modifier::BOLD)
    }

    /// .tui-row.selected — bg=primary, fg=black, bold
    pub fn row_selected(&self) -> Style {
        Style::default()
            .bg(self.primary)
            .fg(Color::Rgb(5, 5, 5))
            .add_modifier(Modifier::BOLD)
    }

    /// .tui-row.active — subtle surface bg + primary light text
    pub fn row_active(&self) -> Style {
        Style::default().bg(self.surface_3).fg(self.primary_light)
    }

    pub fn border(&self) -> Style {
        Style::default().fg(self.border_default)
    }
    pub fn border_active(&self) -> Style {
        Style::default().fg(self.primary)
    }
    pub fn block_title(&self) -> Style {
        Style::default().fg(self.text_tertiary)
    }

    pub fn spinner(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::BOLD)
    }

    // Git semantic styles
    pub fn git_clean(&self) -> Style {
        Style::default().fg(self.success_light)
    }
    pub fn git_dirty(&self) -> Style {
        Style::default().fg(self.danger)
    }
    pub fn git_modified(&self) -> Style {
        Style::default().fg(self.warning)
    }
    pub fn git_untracked(&self) -> Style {
        Style::default().fg(self.success_light)
    }
    pub fn git_deleted(&self) -> Style {
        Style::default().fg(self.danger)
    }
    pub fn git_ahead(&self) -> Style {
        Style::default().fg(self.success_light)
    }
    pub fn git_behind(&self) -> Style {
        Style::default().fg(self.danger)
    }

    pub fn github_stats(&self) -> Style {
        Style::default().fg(self.primary_light)
    }

    pub fn link(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::UNDERLINED)
    }
    pub fn input_prompt(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::BOLD)
    }
    pub fn label(&self) -> Style {
        Style::default()
            .fg(self.text_secondary)
            .add_modifier(Modifier::BOLD)
    }
    pub fn commit_msg(&self) -> Style {
        Style::default()
            .fg(self.text_tertiary)
            .add_modifier(Modifier::ITALIC)
    }
    pub fn project_name(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }
    pub fn branch_name(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    // ════════════════════════════════════════════════════════
    //  TUI COMPONENT HELPERS — Span/Line constructors
    // ════════════════════════════════════════════════════════

    pub fn dot_on<'a>(&self) -> Span<'a> {
        Span::styled(
            DOT_ON,
            Style::default()
                .fg(self.success_light)
                .add_modifier(Modifier::BOLD),
        )
    }
    pub fn dot_warn<'a>(&self) -> Span<'a> {
        Span::styled(DOT_WARN, Style::default().fg(self.warning))
    }
    pub fn dot_err<'a>(&self) -> Span<'a> {
        Span::styled(DOT_ERR, Style::default().fg(self.danger))
    }
    pub fn dot_info<'a>(&self) -> Span<'a> {
        Span::styled(DOT_INFO, Style::default().fg(self.info))
    }
    pub fn dot_off<'a>(&self) -> Span<'a> {
        Span::styled(DOT_OFF, Style::default().fg(self.text_disabled))
    }

    pub fn key_hint<'a>(&self, key: &'a str) -> Span<'a> {
        Span::styled(
            key,
            Style::default()
                .fg(self.text_secondary)
                .bg(self.surface_4)
                .add_modifier(Modifier::BOLD),
        )
    }

    pub fn key_hint_segment<'a>(&self, key: &'a str, label: &'a str) -> Vec<Span<'a>> {
        vec![
            Span::styled("[", self.text_muted()),
            self.key_hint(key),
            Span::styled("]", self.text_muted()),
            Span::styled(" ", Style::default()),
            Span::styled(label, self.text_disabled()),
            Span::styled("  ", Style::default()),
        ]
    }

    pub fn status_seg<'a>(&self, label: &'a str, value: &'a str) -> Vec<Span<'a>> {
        vec![
            Span::styled("[ ", self.status_bar()),
            Span::styled(label, self.status_bar().add_modifier(Modifier::BOLD)),
            Span::styled(": ", self.status_bar()),
            Span::styled(
                value,
                Style::default()
                    .bg(self.primary)
                    .fg(Color::Rgb(5, 5, 5))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ]", self.status_bar()),
            Span::styled("  ", self.status_bar()),
        ]
    }

    pub fn status_bar_line<'a>(
        &self,
        left: Vec<(&'a str, &'a str)>,
        right: Vec<(&'a str, &'a str)>,
    ) -> Line<'a> {
        let mut spans: Vec<Span> = Vec::new();
        for (label, value) in left {
            spans.extend(self.status_seg(label, value));
        }
        spans.push(Span::styled(" ".repeat(4), self.status_bar()));
        for (key, label) in right {
            spans.push(Span::styled("[", self.status_bar()));
            spans.push(Span::styled(
                key,
                Style::default()
                    .bg(self.primary)
                    .fg(Color::Rgb(5, 5, 5))
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ));
            spans.push(Span::styled("] ", self.status_bar()));
            spans.push(Span::styled(label, self.status_bar()));
            spans.push(Span::styled("  ", self.status_bar()));
        }
        Line::from(spans)
    }

    pub fn nav_item<'a>(&self, label: &'a str, active: bool) -> Line<'a> {
        if active {
            Line::from(vec![
                Span::styled(NAV_ACTIVE_PREFIX, self.accent()),
                Span::styled(label, self.accent()),
            ])
        } else {
            Line::from(vec![
                Span::styled(NAV_IDLE_PREFIX, Style::default()),
                Span::styled(label, self.text_muted()),
            ])
        }
    }

    pub fn progress_bar<'a>(&self, filled: u8, total: u8, pct: u8) -> Vec<Span<'a>> {
        let bar: String = (0..total)
            .map(|i| if i < filled { BAR_FULL } else { BAR_LIGHT })
            .collect();
        let pct_str = format!(" {:3}%", pct);
        let bar_style = if pct >= 80 {
            Style::default().fg(self.success_light)
        } else if pct >= 40 {
            Style::default().fg(self.primary)
        } else {
            Style::default().fg(self.text_disabled)
        };
        vec![
            Span::styled(bar, bar_style),
            Span::styled(pct_str, self.text_muted()),
        ]
    }

    pub fn cursor_span<'a>(&self, tick_count: u8) -> Span<'a> {
        let visible = (tick_count / DUR_BASE_TICKS) % 2 == 0;
        if visible {
            Span::styled(CURSOR_BLOCK, Style::default().fg(self.primary))
        } else {
            Span::raw(" ")
        }
    }

    pub fn spinner_span<'a>(&self, tick_count: u8) -> Span<'a> {
        let frame = SPINNER_FRAMES[(tick_count / DUR_FAST_TICKS) as usize % SPINNER_FRAMES.len()];
        Span::styled(frame, self.spinner())
    }

    pub fn tree_item<'a>(&self, label: &'a str, prefix: &'a str, active: bool) -> Line<'a> {
        let prefix_style = self.text_muted();
        let label_style = if active {
            Style::default().fg(self.primary).bg(self.primary_bg)
        } else {
            self.text_dim()
        };
        Line::from(vec![
            Span::styled(prefix, prefix_style),
            Span::styled(label, label_style),
        ])
    }

    pub fn panel_header_line<'a>(&self, label: &'a str) -> Line<'a> {
        Line::from(vec![Span::styled(
            format!("[ {} ]", label),
            Style::default()
                .bg(self.primary)
                .fg(Color::Rgb(5, 5, 5))
                .add_modifier(Modifier::BOLD),
        )])
    }

    pub fn prompt_line<'a>(
        &self,
        user: &'a str,
        path: &'a str,
        input: &'a str,
        tick_count: u8,
    ) -> Line<'a> {
        let mut spans = vec![
            Span::styled(user, self.success()),
            Span::styled(":", self.text_muted()),
            Span::styled(path, self.accent()),
            Span::styled("$ ", self.text_muted()),
            Span::styled(input, self.text()),
        ];
        spans.push(self.cursor_span(tick_count));
        Line::from(spans)
    }
}
