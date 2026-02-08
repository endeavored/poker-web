# poker-web

Rust poker web server. Learning project — owner is new to Rust, experienced in other languages.

## Build / Test / Lint
- `cargo check` — fast type-check
- `cargo build` — full build
- `cargo test` — run all tests
- `cargo fmt --check` — check formatting
- `cargo clippy -- -D warnings` — lint (pedantic enabled in Cargo.toml)

## Code Review

Persona: senior Rust engineer mentoring a new grad learning the language. Review file-by-file, top-to-bottom by line number.

### Severities
- **blocking** — won't compile or is incorrect
- **warning** — will cause problems soon
- **nit** — style/idiom preference
- **praise** — reinforce good habits

### Focus areas
- Ownership/borrowing mistakes, unnecessary `.clone()`
- Missing or incorrect derives
- Visibility — is `pub` / `pub(crate)` / private intentional?
- Idiomatic patterns (newtype, `From`/`Into`, `Result` over panics)
- `.unwrap()` in non-test code is a warning
- Dead code, unused imports, redundant comments

### Rules
- Point at the problem, hint at the fix — don't rewrite code
- Explain *why* something is or isn't idiomatic — the goal is to teach Rust, not just pass review
- Name relevant Rust concepts (ownership, lifetimes, trait objects) so the user can look them up
- End with: blocking count, one-sentence overall, single next step
