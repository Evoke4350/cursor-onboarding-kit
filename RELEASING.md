# Releasing

This project supports two release channels:

1. Official tagged releases (`vX.Y.Z`)
2. Branch snapshot releases (`branch-<branch>-vX.Y.Z`)

Both channels can share the same semver.

## Shared Semver

- Canonical semver is stored in `VERSION`.
- Keep `VERSION` identical across branches when snapshots should stay pinned to one semver line.
- Example: `1.0.0` on `main` and `codex/cursor-ai-2026-02-17`.

## Official Tagged Release

Use this for the canonical public release.

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

Workflow:

- `.github/workflows/release-package.yml`
- Trigger: tag push matching `v*.*.*`

Assets:

- `ai-engineering-workflow-jumpstart-kit-vX.Y.Z.zip`
- `ai-engineering-workflow-jumpstart-docs-vX.Y.Z.zip`
- `SHA256SUMS.txt`

## Branch Snapshot Release

Use this for branch-specific downloadable packages while preserving the same semver.

Workflow:

- `.github/workflows/branch-release-package.yml`
- Trigger: any branch push, or manual `workflow_dispatch`
- Tag format: `branch-<branch-slug>-vX.Y.Z`
- Release name: `vX.Y.Z (<branch-name>)`
- Snapshot releases are marked as prerelease

Assets:

- `ai-engineering-workflow-jumpstart-kit-vX.Y.Z-<branch-slug>.zip`
- `ai-engineering-workflow-jumpstart-docs-vX.Y.Z-<branch-slug>.zip`
- `SHA256SUMS.txt`

## Operational Notes

- Branch snapshot tags are force-updated to the latest commit on that branch.
- This keeps one stable release URL per branch+semver pair while assets track head.
- GitHub also provides built-in source archives (`zip` and `tar.gz`) on each release page.
