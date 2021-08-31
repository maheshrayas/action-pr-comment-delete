
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

        // input : issue number
    // get org
    // get repo
    // secret github_token
    // user
    
    // fn to get the list of comments for the PR
    // curl -H "Authorization: token ${{ secrets.GITHUB_TOKEN }}" -H "Accept: application/vnd.github.VERSION.full+json" https://api.github.com/repos/anzx/platform-secret-management/issues/691/comments
    // get all the comments by user.login = user passed "github-actions[bot]" and get the id and 
    // repos/anzx/platform-secrets-management/pulls/comments/{comment_id}

    println!("INPUT_ORG {:?}",env::var("INPUT_ORG").unwrap());
    println!("printing env {:?}",env::var("INPUT_REPO").unwrap());
    println!("printing INPUT_USER env {:?}",env::var("INPUT_USER").unwrap());

    let res = reqwest::get("https://hyper.rs").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}
