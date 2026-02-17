# AI Output Provenance Record

## Artifact

- Name: `v1.0.0` official release package
- Location: https://github.com/Evoke4350/cursor-onboarding-kit/releases/tag/v1.0.0
- Owner: Project maintainers (represented by Nathaniel Bennett)
- Date: 2026-02-17

## Generation metadata

- Source type: `ai-assisted`
- Model/provider label: `Cursor agent (Codex, GPT-5 family)`
- Prompt style: `template-driven` + `scoped task`
- Inputs used:
  - `.github/workflows/release-package.yml`
  - `.github/workflows/branch-release-package.yml`
  - `RELEASING.md`
  - GitHub Releases metadata/API for `v1.0.0`

## Human controls applied

- [x] Human review completed
- [x] Facts validated against primary sources
- [x] Security/privacy check completed
- [x] Tests run (if code)
- [x] Commit/PR narrative rewritten for clarity
- Reviewer identity/group: `Project Maintainers Review Group (represented by Nathaniel Bennett)`

Evidence:

- Release run: https://github.com/Evoke4350/cursor-onboarding-kit/actions/runs/22090647002
- Asset URLs:
  - https://github.com/Evoke4350/cursor-onboarding-kit/releases/download/v1.0.0/ai-engineering-workflow-jumpstart-kit-v1.0.0.zip
  - https://github.com/Evoke4350/cursor-onboarding-kit/releases/download/v1.0.0/ai-engineering-workflow-jumpstart-docs-v1.0.0.zip
  - https://github.com/Evoke4350/cursor-onboarding-kit/releases/download/v1.0.0/SHA256SUMS.txt

## Confidence and risk

- Confidence: `high`
- Main risk: `none (initial release baseline).`
- Mitigation: `none required for this initial release baseline.`

## Repro note

```bash
gh release view v1.0.0 --json tagName,url,publishedAt,assets
```
