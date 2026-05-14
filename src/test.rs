use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TestResponse {
    name: String,
}

fn main() {
    let req = ureq::get("https://api.github.com/repos/ratatui-org/ratatui").header("User-Agent", "Test");
    if let Ok(res) = req.call() {
        if let Ok(json) = res.into_body().read_json::<TestResponse>() {
            println!("Repo name: {}", json.name);
        } else {
            println!("Failed to parse JSON");
        }
    } else {
        println!("Request failed");
    }
}
