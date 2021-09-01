use futures::future;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct PRComment {
    url: String,
    id: i32,
    user: User,
}
#[derive(Deserialize, Debug)]
struct User {
    login: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let repo = env::var("INPUT_REPO").expect("Missing input parameter: repo");
    let org = env::var("INPUT_ORG").expect("Missing input parameter: org");
    let user = env::var("INPUT_USER").expect("Missing input parameter: user");
    let issue = env::var("INPUT_ISSUE").expect("Missing input parameter: issue");
    let token = format!(
        "token {}",
        env::var("INPUT_GITHUB_TOKEN").expect("Missing input parameter: github_token")
    );

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", token.parse().unwrap());
    headers.insert(
        "Accept",
        "application/vnd.github.VERSION.full+json".parse().unwrap(),
    );
    headers.insert("User-Agent", repo.parse().unwrap());

    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        &org, &repo, &issue
    );
    let res = client.get(url).headers(headers.to_owned()).send().await?;
    let body = res.json::<Vec<PRComment>>().await?;

    let response = future::join_all(
        body.iter()
            .filter(|pr| pr.user.login == user)
            .map(|pr_com| {
                println!("Deleting comment id {}", pr_com.id);
                client
                    .delete(&pr_com.url)
                    .headers(headers.to_owned())
                    .send()
            }),
    )
    .await;

    for b in response {
        match b {
            Ok(b) => println!("Got {} response", b.status()),
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }

    Ok(())
}
