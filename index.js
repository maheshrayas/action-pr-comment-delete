const core = require("@actions/core");
const github = require("@actions/github");
const { Octokit } = require("@octokit/action");
async function run() {
  try {
    const githubToken = core.getInput("github_token");
    const owner = core.getInput("org");
    const user = core.getInput("user");
    const issue = core.getInput("issue");
    const repo = core.getInput("repo");

    const octokit = new Octokit();
    let token = `token ${githubToken}`;
    let req = await octokit.request(
      "GET /repos/{owner}/{repo}/issues/{issue_number}/comments",
      {
        headers: {
          authorization: token,
          Accept: "application/vnd.github.v3+json",
        },
        owner,
        repo,
        issue_number: issue,
      }
    );
    for (const issue of req.data) {
      if (issue.user.login == user) {
        await octokit.request(
          "DELETE /repos/{owner}/{repo}/issues/comments/{comment_id}",
          {
            headers: {
              authorization: token,
              Accept: "application/vnd.github.v3+json",
            },
            owner,
            repo,
            comment_id: `${issue.id}`,
          }
        );
        console.log(
          `Successfully deleted /repos/{owner}/{repo}/issues/comments/${issue.id}`
        );
      }
    }
  } catch (error) {
    core.setFailed(error.message);
  }
}
run();
