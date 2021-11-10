const core = require('@actions/core');
const github = require('@actions/github');
const { Octokit } = require("@octokit/action");
async function run() {
try {
    const githubToken = core.getInput('github_token');
    const org = core.getInput('org');
    const user = core.getInput('user');
    const issue = core.getInput('issue');
    const repo = core.getInput('repo');

    let token = `token ${githubToken}` 
    let req = await Octokit.request('GET /repos/{owner}/{repo}/issues/{issue_number}/comments', {
        headers: {
            authorization: token,
          },
        owner: org,
        repo: repo,
        issue_number: issue
      })

    console.log(`${result.data.length} repos found.`);

  } catch (error) {
    core.setFailed(error.message);
  }
}
run();