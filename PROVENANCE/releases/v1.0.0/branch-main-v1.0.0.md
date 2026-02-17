# AI Output Provenance Record

## Artifact

- Name: `v1.0.0` branch prerelease package (`main`)
- Location: https://github.com/Evoke4350/cursor-onboarding-kit/releases/tag/branch-main-v1.0.0
- Owner: Project maintainers (represented by Nathaniel Bennett)
- Date: 2026-02-17

## Generation metadata

- Source type: `ai-assisted`
- Model/provider label: `Cursor agent (Codex, GPT-5 family)`
- Prompt style: `template-driven` + `scoped task`
- Inputs used:
  - `.github/workflows/branch-release-package.yml`
  - `VERSION`
  - GitHub Releases metadata/API for `branch-main-v1.0.0`

## Human controls applied

- [x] Human review completed
- [x] Facts validated against primary sources
- [x] Security/privacy check completed
- [x] Tests run (if code)
- [x] Commit/PR narrative rewritten for clarity

Evidence:

- Branch release run: https://github.com/Evoke4350/cursor-onboarding-kit/actions/runs/22091512543
- Asset URLs:
  - https://github.com/Evoke4350/cursor-onboarding-kit/releases/download/branch-main-v1.0.0/ai-engineering-workflow-jumpstart-kit-v1.0.0-main.zip
  - https://github.com/Evoke4350/cursor-onboarding-kit/releases/download/branch-main-v1.0.0/ai-engineering-workflow-jumpstart-docs-v1.0.0-main.zip
  - https://github.com/Evoke4350/cursor-onboarding-kit/releases/download/branch-main-v1.0.0/SHA256SUMS.txt

## Confidence and risk

- Confidence: `high`
- Main risk: branch snapshot tags are force-updated on new pushes.
- Mitigation: treat branch prereleases as moving snapshots and verify checksum date before use.

## Repro note

```bash
gh release view branch-main-v1.0.0 --json tagName,url,publishedAt,assets
```
