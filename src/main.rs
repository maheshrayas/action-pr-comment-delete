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
    let config = load_config();
    let client = create_client(&config.repo, &config.token)?;

    let comments = fetch_comments(&client, &config).await?;
    delete_user_comments(&client, &comments, &config.user).await;

    Ok(())
}

struct Config {
    repo: String,
    org: String,
    user: String,
    issue: String,
    token: String,
}

fn load_config() -> Config {
    let repo = env::var("INPUT_REPO").expect("Missing input parameter: repo");
    let org = env::var("INPUT_ORG").expect("Missing input parameter: org");
    let user = env::var("INPUT_USER").expect("Missing input parameter: user");
    let issue = env::var("INPUT_ISSUE").expect("Missing input parameter: issue");
    let token = format!(
        "token {}",
        env::var("INPUT_GITHUB_TOKEN").expect("Missing input parameter: github_token")
    );

    Config { repo, org, user, issue, token }
}

fn create_client(repo: &str, token: &str) -> Result<reqwest::Client, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", token.parse().unwrap());
    headers.insert(
        "Accept",
        "application/vnd.github.VERSION.full+json".parse().unwrap(),
    );
    headers.insert("User-Agent", repo.parse().unwrap());

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
}

async fn fetch_comments(client: &reqwest::Client, config: &Config) -> Result<Vec<PRComment>, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments?per_page=100",
        config.org, config.repo, config.issue
    );
    let res = client.get(&url).send().await?;
    res.json::<Vec<PRComment>>().await.or(Ok(Vec::new()))
}

async fn delete_user_comments(client: &reqwest::Client, comments: &[PRComment], target_user: &str) {
    let user_comments = comments
        .iter()
        .filter(|pr| pr.user.login == target_user);

    let delete_futures = user_comments.map(|pr_com| {
        println!("Deleting comment id {}", pr_com.id);
        client.delete(&pr_com.url).send()
    });

    let responses = future::join_all(delete_futures).await;

    for response in responses {
        match response {
            Ok(res) => println!("Got {} response", res.status()),
            Err(err) => eprintln!("Got an error: {}", err),
        }
    }
}
