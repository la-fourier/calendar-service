// use std::env::args;

// fn main() {
//     let inp = args();
// }


use reqwest;
use json::parse;

fn main() -> Result<(), reqwest::Error> {
    // GitHub Personal Access Token (ersetze dies durch dein eigenes Token)
    let token = "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";

    // GitHub API-Endpunkt für Issues
    let url = "https://api.github.com/repos/la-fourier/calendar-service/issues";

    // JSON-Payload für das neue Issue
    let payload = json::parse(r#"{
        "title": "Neues Issue",
        "body": "Hier ist der Text des neuen Issues."
    }"#);

    // Erstelle einen HTTP-Client
    let client = reqwest::Client::new();

    // Sende die Anfrage an die GitHub API
    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .json(&payload)
        .send()?;

    // Überprüfe die Antwort
    if response.status().is_success() {
        println!("Issue erfolgreich erstellt!");
    } else {
        println!("Fehler beim Erstellen des Issues: {:?}", response.text());
    }

    Ok(())
}
