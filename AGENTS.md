# Agent Commit Workflow (Repository Policy)

Use this repository-local Git identity:

- `user.name`: `Nathaniel Bennett`
- `user.email`: `nathanib@pm.me`

Use these repository-local aliases:

- `git ci -m "<message>"` -> `git commit -s -m "<message>"`
- `git cia` -> `git commit --amend --no-edit -s`

Required for every contribution:

1. Include DCO sign-off on each commit (`Signed-off-by` trailer).
2. Pass CLA checks on pull requests (`CLA/ICLA.md` or approved `CLA/CCLA.md` path).
3. Follow license scope in `LICENSES.md`.
4. If runtime behavior changed, add/update tests; docs/templates/process-only work can use `N/A` with reason.

Fast pre-push check:

```bash
for c in $(git rev-list origin/main..HEAD); do
  git show -s --format=%B "$c" | grep -q '^Signed-off-by:' || echo "Missing sign-off: $c"
done
```

Fix missing sign-offs:

- Last commit: `git cia`
- Branch: `git rebase --signoff origin/main`

Lab note:

- Some lab code is intentionally broken for teaching purposes. Preserving known bugs can be correct.
