# Integration Plan: Module-by-Module Port

**Purpose**: Compose the Agentic Blackboard from source repos, concept by concept, line by line.

---

## Overview

8 domains → 8 modules → 29 tasks → 113 acceptance criteria

All modules are **composable** — can be used independently or together.

---

## Module 1: Security (`workshop/modules/security/`)

### Source: `~/agno/libs/agno/agno/guardrails/`

#### File: `guardrail.rs`

**Port from**: `~/agno/libs/agno/agno/guardrails/base.py` (lines 8-20)

```rust
// Original Python:
// class BaseGuardrail(ABC):
//     @abstractmethod
//     def check(self, run_input: RunInput) -> None: ...
//     @abstractmethod
//     async def async_check(self, run_input: RunInput) -> None: ...

// Rust port:
pub trait Guardrail {
    fn check(&self, input: &str) -> Result<(), GuardrailError>;
    async fn async_check(&self, input: &str) -> Result<(), GuardrailError>;
}
```

**Concept**: Abstract base for all guardrails with sync/async check methods.

#### File: `taint.rs`

**Port from**: Universalis doc (conceptual) + `~/agno/libs/agno/agno/guardrails/pii.py`

```rust
// Concept from Universalis:
// source_output{ pred: "read_file", field: "contents" }
// sink_spec{ pred: "curl_post", data_field: "data", dest_field: "url" }
// path{ source: S, var: V }  // transitive closure
// violation{ source: S, dest: D }  // when tainted reaches untrusted sink

pub struct TaintTracker {
    sources: Vec<SourceSpec>,
    sinks: Vec<SinkSpec>,
    tainted: bool,
}

impl TaintTracker {
    pub fn mark_tainted(&mut self, source: &str) { ... }
    pub fn check_sink(&self, sink: &str) -> Result<(), TaintError> { ... }
}
```

**Concept**: Track data flow from protected sources to exfil sinks.

#### File: `policy.rs`

**Port from**: New (YAML-based configuration)

```rust
// sources.yaml:
// sources:
//   - pattern: ".secrets/*"
//     taint: high

#[derive(Deserialize)]
pub struct SourceSpec {
    pub pattern: String,
    pub taint: TaintLevel,
}

#[derive(Deserialize)]
pub struct SinkSpec {
    pub command: String,
    pub block_if_tainted: bool,
}
```

**Concept**: Declarative security policy in YAML.

---

## Module 2: Cursor (`workshop/modules/cursor/`)

### Source: `~/Dicklesworthstone/coding_agent_session_search/src/connectors/cursor.rs`

#### File: `sqlite.rs`

**Port from**: Lines 156-239

```rust
// Original (Rust):
// fn find_cursor_dbs() -> Vec<PathBuf> {
//     let base = dirs::data_dir()?.join("Cursor/User/globalStorage");
//     // Find all state.vscdb files
// }

pub fn find_cursor_databases() -> Result<Vec<PathBuf>> {
    let base = dirs::data_dir()
        .ok_or_else(|| anyhow!("Cannot find data directory"))?
        .join("Cursor/User/globalStorage");

    let mut dbs = Vec::new();
    for entry in fs::read_dir(base)? {
        // Check for state.vscdb
    }
    Ok(dbs)
}
```

**Concept**: Find Cursor SQLite databases in platform-specific locations.

#### File: `composer.rs`

**Port from**: Lines 422-593

```rust
// Original: Parse composerData:{uuid} entries
// Key format: composerData:550e8400-e29b-41d4-a716-446655440000
// Value format: JSON with fullConversationHeadersOnly, createdAt, etc.

#[derive(Deserialize)]
pub struct ComposerData {
    pub full_conversation_headers_only: Vec<MessageHeader>,
    pub created_at: i64,
    pub last_updated_at: i64,
}

pub fn parse_composer_data(db: &Connection, uuid: &str) -> Result<ComposerData> {
    let key = format!("composerData:{}", uuid);
    let value: String = db.query_row(
        "SELECT value FROM cursorDiskKV WHERE key = ?",
        [&key],
        |row| row.get(0)
    )?;
    serde_json::from_str(&value).map_err(Into::into)
}
```

**Concept**: Parse Composer conversation data from Cursor's SQLite.

#### File: `bubble.rs`

**Port from**: Lines 245-282

```rust
// Original: Parse bubbleId:{composer}:{bubble} entries
// For Cursor v0.40+, bubbles are stored separately

#[derive(Deserialize)]
pub struct Bubble {
    pub text: String,
    pub role: String,  // "user" or "assistant"
    pub created_at: i64,
}

pub fn parse_bubbles(db: &Connection, composer_id: &str) -> Result<Vec<Bubble>> {
    // Query all bubbleId:{composer_id}:* entries
}
```

**Concept**: Extract individual messages from Cursor v0.40+ format.

---

## Module 3: Memory (`workshop/modules/memory/`)

### Source: `~/agno/libs/agno/agno/db/schemas/memory.py` + `manager.py`

#### File: `schema.rs`

**Port from**: `memory.py` lines 8-58

```rust
// Original Python:
// @dataclass
// class UserMemory:
//     memory: str
//     memory_id: Optional[str]
//     topics: Optional[List[str]]
//     user_id: Optional[str]
//     created_at: Optional[int]
//     updated_at: Optional[int]
//     feedback: Optional[str]
//     agent_id: Optional[str]
//     team_id: Optional[str]

#[derive(Serialize, Deserialize, Clone)]
pub struct UserMemory {
    pub memory: String,
    pub memory_id: Option<String>,
    pub topics: Option<Vec<String>>,
    pub user_id: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub feedback: Option<String>,
    pub agent_id: Option<String>,
    pub team_id: Option<String>,
    // Blackboard extension:
    pub taint: Option<Vec<String>>,
    pub code_ref: Option<CodeReference>,
}
```

**Concept**: Memory schema with Agno fields + Blackboard extensions.

#### File: `manager.rs`

**Port from**: `manager.py` lines 42-1543

```rust
// Original: Full CRUD + search + optimization
// Key methods:
// - add_memory()
// - get_memories()
// - search_memories()
// - optimize_memories()

pub struct MemoryManager {
    db: Connection,
    config: MemoryConfig,
}

impl MemoryManager {
    pub fn add(&self, memory: UserMemory) -> Result<String> { ... }
    pub fn get(&self, memory_id: &str) -> Result<Option<UserMemory>> { ... }
    pub fn search(&self, query: &str) -> Result<Vec<UserMemory>> { ... }
    pub fn by_topic(&self, topic: &str) -> Result<Vec<UserMemory>> { ... }
}
```

**Concept**: CRUD operations for memory storage.

---

## Module 4: Pipeline (`workshop/modules/pipeline/`)

### Source: `~/arscontexta/generators/features/processing-pipeline.md`

#### File: `phases.rs`

**Port from**: Lines 12-89

```rust
// Original: 4-phase pipeline
// Capture → Process → Connect → Verify

pub enum Phase {
    Capture,   // Zero-friction capture to sawdust/inbox/
    Cut,       // Extract atomic insight
    Carve,     // Find connections
    Chamfer,   // Update older notes
    Check,     // Validate
}

pub struct Pipeline {
    current_phase: Phase,
    queue: VecDeque<Task>,
}

impl Pipeline {
    pub fn advance(&mut self) -> Option<Task> { ... }
    pub fn run_phase(&mut self, phase: Phase) -> Result<()> { ... }
}
```

**Concept**: Multi-phase processing with quality gates.

#### File: `queue.rs`

**Port from**: Lines 117-176

```rust
// Original: JSON task queue tracks each note through phases

#[derive(Serialize, Deserialize)]
pub struct QueuedTask {
    pub id: String,
    pub source_path: PathBuf,
    pub current_phase: Phase,
    pub status: TaskStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct TaskQueue {
    path: PathBuf,  // sawdust/queue.json
}

impl TaskQueue {
    pub fn push(&mut self, task: QueuedTask) -> Result<()> { ... }
    pub fn pop(&mut self) -> Result<Option<QueuedTask>> { ... }
    pub fn advance(&mut self, task_id: &str, new_phase: Phase) -> Result<()> { ... }
}
```

**Concept**: Persistent task queue for pipeline state.

---

## Module 5: Setup (`workshop/modules/setup/`)

### Source: `~/arscontexta/generators/claude-md.md` + `~/arscontexta/generators/features/*.md`

#### File: `detect.rs`

**Port from**: arscontexta concepts

```rust
pub struct Environment {
    pub os: String,
    pub has_gum: bool,
    pub has_ripgrep: bool,
    pub has_uvx: bool,
    pub cursor_installed: bool,
}

pub fn detect_environment() -> Result<Environment> {
    Ok(Environment {
        os: std::env::consts::OS.to_string(),
        has_gum: which::which("gum").is_ok(),
        has_ripgrep: which::which("rg").is_ok(),
        has_uvx: which::which("uvx").is_ok(),
        cursor_installed: detect_cursor(),
    })
}
```

**Concept**: Detect available tools and environment.

#### File: `derive.rs`

**Port from**: arscontexta derivation engine (lines 116-125)

```rust
// Original: 8 configuration dimensions with cascade resolution

#[derive(Default)]
pub struct DerivedConfig {
    pub work_type: WorkType,
    pub code_refs: CodeRefMode,
    pub location: PathBuf,
    pub security_level: SecurityLevel,
    // ... 4 more dimensions
}

pub fn derive_from_answers(answers: HashMap<String, String>) -> DerivedConfig {
    let mut config = DerivedConfig::default();

    // Cascade resolution: explicit > implicit > later > earlier > specific > general
    if let Some(wt) = answers.get("work_type") {
        config.work_type = WorkType::from_str(wt);
    }
    // ... more derivation logic

    config
}
```

**Concept**: Derive configuration from conversational answers.

#### File: `generate.rs`

**Port from**: Multiple arscontexta feature files

```rust
pub fn generate_workshop(config: &DerivedConfig, path: &Path) -> Result<()> {
    // Create folder structure
    fs::create_dir_all(path.join("bench"))?;
    fs::create_dir_all(path.join("shavings"))?;
    fs::create_dir_all(path.join("sawdust/sessions"))?;
    fs::create_dir_all(path.join(".workshop/tools/core"))?;
    fs::create_dir_all(path.join(".workshop/policy"))?;

    // Generate identity.md
    let identity = generate_identity(config);
    fs::write(path.join("bench/identity.md"), identity)?;

    // Generate policy
    let policy = generate_policy(config);
    fs::write(path.join(".workshop/policy/sources.yaml"), policy.sources)?;
    fs::write(path.join(".workshop/policy/sinks.yaml"), policy.sinks)?;

    Ok(())
}
```

**Concept**: Generate workshop files from derived configuration.

---

## Module 6: Tools (`workshop/modules/tools/`)

### Source: `~/arscontexta/generators/features/templates.md`

#### File: `format.rs`

**Port from**: Lines 12-49

```rust
// Original: YAML frontmatter with schema validation

#[derive(Deserialize)]
pub struct ToolFrontmatter {
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub performance: PerformanceBudget,
}

pub fn parse_tool(content: &str) -> Result<(ToolFrontmatter, String)> {
    // Extract YAML between --- delimiters
    // Validate required fields
    // Return frontmatter + body
}
```

**Concept**: Tool definition format with YAML frontmatter.

#### File: `templates.rs`

**Port from**: Lines 17-82

```rust
// Original: Feature blocks - composable template system

pub struct FeatureBlock {
    pub name: String,
    pub template: String,
    pub variables: HashMap<String, String>,
}

impl FeatureBlock {
    pub fn render(&self, vars: &HashMap<String, String>) -> String {
        let mut result = self.template.clone();
        for (key, value) in vars {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }
}
```

**Concept**: Composable templates with variable substitution.

---

## Module 7: CLI (`workshop/modules/cli/`)

### Source: `~/Dicklesworthstone/destructive_command_guard/src/` + `~/Dicklesworthstone/xf/src/`

#### File: `perf.rs`

**Port from**: `perf.rs` lines 35-100

```rust
// Original: Tiered latency budgets with panic/warning thresholds

#[derive(Clone)]
pub struct PerformanceBudget {
    pub target: Duration,
    pub warning: Duration,
    pub panic: Duration,
}

impl PerformanceBudget {
    pub fn quick_reject() -> Self {
        Self {
            target: Duration::from_micros(1),
            warning: Duration::from_micros(10),
            panic: Duration::from_micros(50),
        }
    }

    pub fn fast_path() -> Self {
        Self {
            target: Duration::from_micros(75),
            warning: Duration::from_micros(200),
            panic: Duration::from_micros(500),
        }
    }

    pub fn check(&self, elapsed: Duration) -> BudgetResult { ... }
}
```

**Concept**: Performance budget checking.

#### File: `search.rs`

**Port from**: `xf/src/hybrid.rs` lines 1-859

```rust
// Original: Hybrid lexical + semantic search with RRF fusion

pub struct HybridSearch {
    lexical_index: TantivyIndex,  // ripgrep-style
    vector_index: Option<VectorIndex>,  // optional semantic
}

impl HybridSearch {
    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let lexical = self.lexical_search(query)?;
        let vector = self.vector_search(query)?;

        // Reciprocal Rank Fusion
        let fused = self.rrf_fusion(lexical, vector);
        Ok(fused)
    }
}
```

**Concept**: Sub-millisecond hybrid search.

#### File: `output.rs`

**Port from**: New (for Blackboard)

```rust
pub enum OutputMode {
    Human,  // Rich terminal output
    Robot,  // JSON for scripting
}

pub fn format_output(mode: OutputMode, result: &Result) -> String {
    match mode {
        OutputMode::Human => format_human(result),
        OutputMode::Robot => serde_json::to_string(result).unwrap(),
    }
}
```

**Concept**: Dual output modes for CLI.

---

## Module 8: Proof (`workshop/modules/proof/`)

### Source: `~/showboat/` + `~/chartroom/`

#### File: `showboat.rs`

**Port from**: `showboat/main.go`, `extract.go`, `verify.go`

```rust
// Original: Executable documents with code extraction and verification

pub struct ShowboatDoc {
    pub path: PathBuf,
    pub uuid: Uuid,
}

impl ShowboatDoc {
    pub fn init(path: &Path, title: &str) -> Result<Self> { ... }
    pub fn note(&mut self, text: &str) -> Result<()> { ... }
    pub fn exec(&mut self, lang: &str, code: &str) -> Result<String> { ... }
    pub fn image(&mut self, path: &Path, alt: Option<&str>) -> Result<()> { ... }
    pub fn verify(&self) -> Result<Vec<Diff>> { ... }
    pub fn pop(&mut self) -> Result<()> { ... }
}
```

**Concept**: Executable markdown documents.

#### File: `chart.rs`

**Port from**: `chartroom/` CLI

```rust
// Original: Charts with automatic alt-text generation

pub fn create_chart(
    data: &[DataRow],
    chart_type: ChartType,
    options: ChartOptions,
) -> Result<(PathBuf, String)> {
    // Use chartroom via UVX
    let output = Command::new("uvx")
        .args(["chartroom", chart_type.to_str(), "--format", "markdown"])
        .output()?;

    let path = parse_path(&output)?;
    let alt = parse_alt(&output)?;

    Ok((path, alt))
}
```

**Concept**: Chart generation with accessibility.

---

## Composition

### File: `compose.rs`

```rust
pub struct Workshop {
    security: security::TaintTracker,
    cursor: cursor::CursorReader,
    memory: memory::MemoryManager,
    pipeline: pipeline::Pipeline,
    setup: setup::SetupEngine,
    tools: tools::ToolRegistry,
    cli: cli::CliEngine,
    proof: proof::ShowboatIntegration,
}

impl Workshop {
    pub fn bootstrap(config: DerivedConfig) -> Result<Self> {
        let path = &config.location;

        Ok(Self {
            security: security::TaintTracker::from_policy(path)?,
            cursor: cursor::CursorReader::new()?,
            memory: memory::MemoryManager::new(path)?,
            pipeline: pipeline::Pipeline::new(path)?,
            setup: setup::SetupEngine::new(config),
            tools: tools::ToolRegistry::from_dir(path)?,
            cli: cli::CliEngine::new(),
            proof: proof::ShowboatIntegration::new(path)?,
        })
    }

    pub fn run_command(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Cut { source, options } => {
                self.security.check_read(&source)?;
                let insight = self.pipeline.cut(&source, options)?;
                self.memory.add(insight)?;
                self.proof.exec("cut", &format!("{:?}", insight))?;
            }
            // ... other commands
        }
        Ok(())
    }
}
```

---

## Summary

| Module | Files | Source Lines | Port Lines | Complexity |
|--------|-------|--------------|------------|------------|
| security | 3 | ~150 | ~200 | Medium |
| cursor | 3 | ~400 | ~350 | High |
| memory | 2 | ~1600 | ~800 | High |
| pipeline | 2 | ~300 | ~250 | Medium |
| setup | 3 | ~500 | ~400 | Medium |
| tools | 2 | ~100 | ~150 | Low |
| cli | 3 | ~900 | ~600 | High |
| proof | 2 | ~300 | ~250 | Low |

**Total**: ~3000 lines of ported Rust code from ~4250 lines of source.

---

## Next Step

Execute Ralph loop:
```bash
while :; do cat specs/blackboard-bootstrap/PROMPT.md | claude-code ; done
```

Each iteration implements ONE task from this integration plan.
