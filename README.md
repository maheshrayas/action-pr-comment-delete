
<h1 align="center">
  <p align="center">action-pr-comment-delete</p>
</h1>
<div align="center">
  <a alt="Action pulled"><img src="https://img.shields.io/docker/pulls/maheshrayas/pr-comment-deleter.svg" /></a>
  </div>

When a bot or user comments on a Github Pull request, the comments keep getting added for the subsequent push on the same PR. This Github action can be used in Github workflow prior to the GitHub Job which actually comments on the PR.

## Examples:

```bash
jobs:
  clean:
    runs-on: macos-latest
    steps:
      - name: pr-deleter
        uses: maheshrayas/action-pr-comment-delete@v2.0
        with:
          github_token: '${{ secrets.GITHUB_TOKEN }}'
          org: <orgname>
          repo: <repo>
          user: 'github-actions[bot]' #commented by the userid
          issue: '${{github.event.number}}'
```
