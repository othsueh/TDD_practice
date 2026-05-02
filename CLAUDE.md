# CLAUDE.md — Potter Kata TDD Practice (Rust)

This repo is a TDD learning project using the Potter Kata problem.
The goal is to practice the Red-Green-Refactor cycle in Rust, one small step at a time.

Success = walking through the full Potter Kata test suite using strict TDD discipline,
with every commit representing exactly one meaningful step (Red, Green, or Refactor).

---

## Absolute Rules (Claude must follow these without exception)

- **Never commit directly to main** — always branch → PR → merge
- **Never skip hooks** — do not use `--no-verify` or `-c commit.gpgsign=false`
- **Never force-push** — if you need to rewrite history, stop and ask
- **Never delete files or branches without confirmation** — ask first if uncertain
- **Never expose secrets** — do not commit `.env`, credentials, or tokens
- **Never run `rm -rf` on directories** — always identify files specifically

When in doubt about a destructive or irreversible action, stop and ask.

---

## TDD Rules (must follow every cycle)

- **Red first** — write a failing test before any implementation code
- **Minimum Green** — write the simplest code that makes the test pass (Fake It if needed)
- **One test at a time** — never add a second test before the first is green
- **Commit at each phase** — separate commits for Red, Green, and Refactor
- **No premature abstraction** — resist writing general logic until triangulation forces it

### Commit message format for TDD phases

```
red: <what test was added and why it fails>
green: <what minimal implementation was added>
refactor: <what was cleaned up, all tests still pass>
```

---

## Git & GitHub Workflow

### Branch naming

```
feat/<short-description>    # new feature / new test case
fix/<short-description>     # bug fix
refactor/<short-description>
```

### Commit message format

```
<type>: <short summary in imperative mood>

[optional body — explain *why*, not *what*]

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
```

### After every git operation

Run `git log --oneline -5` to confirm the result landed.

---

## Project-Specific Configuration

### Tech stack

- Language: Rust
- Package manager: Cargo
- Test command: `cargo test`
- Lint/format command: `cargo fmt && cargo clippy`
- Build command: `cargo build`

### Repository info

- `owner`: othsueh
- `repo`: TDD_practice (potter_kata subcrate)
- `default_branch`: main

---

## Potter Kata — Problem Rules

**Goal**: Calculate the price of a shopping cart of Harry Potter books, with discounts for buying different titles.

| Distinct titles in a group | Discount |
|---------------------------|----------|
| 1 book                    | 0%       |
| 2 different books         | 5%       |
| 3 different books         | 10%      |
| 4 different books         | 20%      |
| 5 different books         | 25%      |

- Each book costs 8 EUR
- The cart may contain multiple copies of the same title
- Goal: find the **minimum price** (optimal grouping)

---

## Roles

- **Student (user)**: Decides the next test case — what behavior to test and why
- **Claude**: Implements the test in Rust, runs it, commits each Red/Green/Refactor step

Claude should prompt the student before each cycle:
> "The current tests pass. What's the next behavior you want to test?"

---

## Slash Commands

Slash commands live in `.claude/commands/*.md`. Each file is one command.
Build them incrementally — when the same operation repeats twice, make it a command.

### Commands in this repo

| Command | What it does |
|---------|--------------|
| `/tdd-cycle` | Run `cargo test`, report Red/Green status, prompt for next step |
| `/commit` | Stage, commit with correct TDD phase prefix, push |
| `/sync-main` | Checkout main, pull, verify latest commit |

### When to create a new command

- The same sequence of steps has been explained more than once
- A multi-step workflow has a clear start and end
- An operation must always happen in the same order (e.g., fmt → clippy → test → commit)

### How to write a slash command

```markdown
# .claude/commands/my-command.md

One-line description.

## Steps

1. **Step name** — what to do and which tool/command to use
2. ...
3. **Validate** — always end with a verification step
```

---

## Multi-Agent Patterns

Use parallel agents only when tasks are **independent** (no shared state, no ordering requirement).

### When to parallelize in this project

```
✅ Run cargo test and cargo clippy simultaneously
✅ Research Rust idioms in parallel with reading existing test output
✅ Analyze multiple test case scenarios before deciding the next one

❌ Writing a test while also writing the implementation (TDD requires strict ordering)
❌ Two agents editing the same src/lib.rs
```

### Pattern: parallel research → serial TDD cycle

```
Main agent
├── spawn Agent A → research Rust idiom for this data structure
├── spawn Agent B → look up discount grouping algorithm patterns
└── [wait for both] → synthesize, then write the next test
```

**Key rules:**
- Give each research agent its own worktree (`isolation: "worktree"`) to avoid conflicts
- Only the main agent writes code and commits — sub-agents do research only
- Run `/worktree-cleanup` after any parallel session

---

## Self-Improvement Protocol

Claude should propose improvements to this file and `.claude/commands/` when it spots friction:

1. **Repeated explanation** → write a new slash command
2. **Repeated mistake** → add a rule to TDD Rules or Absolute Rules
3. **Repeated workflow** → document it in the relevant section
4. **Useful refactoring pattern discovered** → add to Test Case Log notes

To apply an improvement:
1. Edit `CLAUDE.md` or add/edit a command file
2. Commit directly with message: `chore: update CLAUDE.md — <what changed and why>`
3. Note the change to the user so they can review it

The goal: each session leaves the repo slightly better equipped than before.

---

## CLAUDE.md Maintenance

Update immediately when:
- A TDD rule needs to be added based on a mistake
- A new test case is added — note it in the test case log below
- A refactoring pattern is discovered
- A slash command is added or changed — update the commands table

**Update format:**

```markdown
## [Section]
- **[Date] [what changed]**: [why it matters / what broke without it]
```

### Test Case Log

| # | Description | Status |
|---|-------------|--------|
| 1 | 空購物車 → 0 EUR | ✅ |
| 2 | 1 本書 → 8 EUR | ✅ |
| 3 | 2 本同書 → 16 EUR（無折扣） | ✅ |
| 4 | 2 本不同書 → 15.2 EUR（5% off） | ✅ |
| 5 | 2 本同 + 1 本不同 → 23.2 EUR（最優分組） | ✅ |
| 6 | 3 本全不同 → 21.6 EUR（10% off） | ✅ |
| 7 | 4 本全不同 → 25.6 EUR（20% off） | ✅ |
| 8 | 5 本全不同 → 30.0 EUR（25% off） | ✅ |
| 9 | 貪心陷阱 `[1,1,2,2,3,3,4,5]` → 51.2 EUR（4+4 優於 5+3） | ✅ |
| 10 | 8 本全同 → 64.0 EUR（無折扣） | ✅ |
| 11 | 貪心陷阱 ×3 → 153.6 EUR（3 次 5+3 全換成 4+4） | ✅ |
