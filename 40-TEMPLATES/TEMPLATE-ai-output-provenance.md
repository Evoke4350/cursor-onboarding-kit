# AI Output Provenance Record

Use this when you want transparent labels for AI-assisted or AI-generated artifacts.

## Artifact

- Name: <doc/code output>
- Location: <path/link>
- Owner: <person/team>
- Date: <YYYY-MM-DD>

## Generation metadata

- Source type: `human-authored | ai-assisted | ai-generated`
- Model/provider label: <e.g., "Robo", "Cursor agent">
- Prompt style: <discussion-first / scoped task / template-driven>
- Inputs used: <files, tickets, links>

## Human controls applied

- [ ] Human review completed
- [ ] Facts validated against primary sources
- [ ] Security/privacy check completed
- [ ] Tests run (if code)
- [ ] Commit/PR narrative rewritten for clarity

## Confidence and risk

- Confidence: <low/medium/high>
- Main risk: <hallucination/drift/stale context/etc>
- Mitigation: <how risk was reduced>

## Repro note

<How another engineer can reproduce or audit this output>
