# Releasing

This project publishes versioned release assets from Git tags.

## Trigger a Release

1. Create a semantic version tag:

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

2. GitHub Actions workflow:

- `.github/workflows/release-package.yml`
- Trigger: tag push matching `v*.*.*`

## Published Assets

Each release uploads:

- `ai-engineering-workflow-jumpstart-kit-vX.Y.Z.zip` (full repository package)
- `ai-engineering-workflow-jumpstart-docs-vX.Y.Z.zip` (Markdown docs bundle)
- `SHA256SUMS.txt` (checksums for both zip files)

GitHub also provides built-in source archives (`zip` and `tar.gz`) on the release page.

## Notes

- The docs bundle is a zip-first distribution path.
- PDF packaging can be added later as an optional release job when the document set is finalized.
