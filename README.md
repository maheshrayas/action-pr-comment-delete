# action-pr-comment-delete

When a bot or user comments on a Github Pull request, the comments keep getting added for the subsequent push on the same PR. This Github action can be used in Github workflow prior to the GitHub Job which actually comments on the PR.

## Examples:

```bash
jobs:
  clean:
    runs-on: ubuntu-latest
    steps:
      - name: pr-deleter
        uses: maheshrayas/action-pr-comment-delete@v1
        with:
          github_token: '${{ secrets.GITHUB_TOKEN }}'
          org: <orgname>
          repo: <repo>
          user: 'github-actions[bot]' #commented by the userid
          issue: '${{github.event.number}}'
```
