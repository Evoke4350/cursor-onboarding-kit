# Generic Content Ingest (Jira, Confluence, Figma)

This module documents a safe, generic pattern for pulling external context into your local blackboard using `curl` + tokens.

## Principles

- Tokens come from environment variables only.
- Raw API payloads stay local and excluded from git.
- Convert raw payloads into sanitized markdown summaries for sharing.
- Never paste secrets into prompts.
- Treat large JSON as data-processing tasks, not copy/paste context.

## Local Folder Pattern

Use a private local path:

- `.agentic-blackboard/private/raw/`
- `.agentic-blackboard/private/normalized/`

Mark these local-only in `.git/info/exclude`.

## Required Environment Variables (Examples)

- `JIRA_BASE_URL`
- `JIRA_EMAIL`
- `JIRA_API_TOKEN`
- `CONFLUENCE_BASE_URL`
- `CONFLUENCE_EMAIL`
- `CONFLUENCE_API_TOKEN`
- `FIGMA_TOKEN`

## Example: Jira Issue Fetch

```bash
curl -sS \
  -u "$JIRA_EMAIL:$JIRA_API_TOKEN" \
  -H "Accept: application/json" \
  "$JIRA_BASE_URL/rest/api/3/issue/UK-1234" \
  -o ".agentic-blackboard/private/raw/jira-UK-1234.json"
```

## Example: Confluence Page Fetch

```bash
curl -sS \
  -u "$CONFLUENCE_EMAIL:$CONFLUENCE_API_TOKEN" \
  -H "Accept: application/json" \
  "$CONFLUENCE_BASE_URL/wiki/rest/api/content/<PAGE_ID>?expand=body.storage,version" \
  -o ".agentic-blackboard/private/raw/confluence-<PAGE_ID>.json"
```

## Example: Figma File JSON Fetch

```bash
curl -sS \
  -H "X-Figma-Token: $FIGMA_TOKEN" \
  "https://api.figma.com/v1/files/<FILE_KEY>" \
  -o ".agentic-blackboard/private/raw/figma-<FILE_KEY>.json"
```

## Normalize For Agent Consumption

After fetching raw JSON:

1. Extract only relevant sections (ticket summary, acceptance criteria, key UI nodes).
2. Write a sanitized markdown note under `10-active/`.
3. Link source artifacts by filename, not by pasting full raw JSON.

## Large Payload Warning (Figma/Jira/Confluence)

Some exports can be 3MB+ and can degrade agent quality if pasted directly.

Use a REPL or script first, then pass only a compact summary.

Recommended operator instruction:

`Use Python REPL (or jq) to inspect this JSON and return a concise summary table before reading full payload sections.`

### Example: quick payload sizing

```bash
python3 - <<'PY'
from pathlib import Path
import json
p = Path(".agentic-blackboard/private/raw/figma-<FILE_KEY>.json")
size_mb = p.stat().st_size / (1024 * 1024)
print(f"size_mb={size_mb:.2f}")
data = json.loads(p.read_text())
print("top_keys=", list(data.keys())[:20])
PY
```

### Example: selective extraction

```bash
python3 - <<'PY'
from pathlib import Path
import json
p = Path(".agentic-blackboard/private/raw/figma-<FILE_KEY>.json")
data = json.loads(p.read_text())
doc = data.get("document", {})
children = doc.get("children", [])
print("document_name:", data.get("name"))
print("top_level_nodes:", len(children))
for n in children[:10]:
    print("-", n.get("name"), n.get("type"))
PY
```

Rule of thumb:

- REPL/script first
- summary second
- targeted deep reads last

## Security Guardrails

- Do not commit raw exports.
- Do not log token values.
- Rotate tokens routinely.
- If sharing outside project scope, redact identifiers.
