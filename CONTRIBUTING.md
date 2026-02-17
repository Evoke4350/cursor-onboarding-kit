# Contributing

## Ground Rules

- Send focused PRs with a clear problem statement.
- Keep changes small enough to review quickly.
- Add or update tests when runtime behavior changes.
- Docs/templates/process-only changes can mark testing as `N/A` with a one-line reason in the PR.
- Some lab code is intentionally broken for pedagogy; preserving known bugs can be the correct outcome for this repository.

## Licensing Model (Read First)

This repository uses mixed licensing:

- Default repository license: MIT (`LICENSE`)
- Full scope map: `LICENSES.md`
- Directory-specific overrides apply where present.

By submitting a contribution, you agree your changes are licensed under
the applicable license for the files you modify.

## DCO Sign-Off (Required)

This project uses the Developer Certificate of Origin 1.1 (`DCO`).
Every commit must include a `Signed-off-by` trailer.

Use:

```bash
git commit -s -m "your message"
```

For an existing commit:

```bash
git commit --amend --signoff
```

The trailer format must match your commit identity:

```text
Signed-off-by: Your Name <your.email@example.com>
```

## CLA (Required)

This project uses an inbound=outbound CLA model:

- Individual CLA: `CLA/ICLA.md`
- Corporate CLA for employer-owned work: `CLA/CCLA.md`
- Process details: `CLA/README.md`
- Corporate authorization ledger: `CLA/COMPANIES.md`

Rules:

- Individual contributors must accept the ICLA.
- If your employer may own your contribution, your organization must complete
  the CCLA onboarding path in `CLA/README.md`.
- Anonymous and pseudonymous identities are not accepted.
- A government-issued legal name is not required, but your name and email must
  identify a real accountable person.

## Enforcement

PR checks enforce CLA acceptance, and maintainers verify CCLA authorization when
applicable. DCO sign-off is still required on each commit.
