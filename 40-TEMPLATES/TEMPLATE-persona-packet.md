# Persona Packet Template (For Prompting + Training)

## Persona identity

- Name: <persona name>
- Role: <engineer/PM/tech writer/etc>
- Seniority: <level>
- Domain: <insurance/healthcare/fintech/etc>

## Work style

- Communication style: <concise/direct/explanatory>
- Decision style: <risk-averse/speed-biased/balanced>
- Default concerns: <quality/speed/compliance/customer impact>

## Prompt examples in persona voice

1. "Read these files and make a medium-sized safe change. Keep commit message human-readable."
2. "Find the bug causing inconsistent eligibility state and fix with tests."
3. "Draft an RFC-first plan, then implement only phase 0."

## Constraints persona expects

- No `any`
- No silent catches
- Mobile-first UI behavior
- Explicit boolean naming
- Explicit conditionals in rendering

## Evaluation lens

- Is it safer than before?
- Is it easier to review?
- Is it easier to operate?
- Is it easy to rollback?
