# poker-web
Rust poker web server. Learning project — owner is new to Rust, experienced in other languages.

## Build / Test / Lint
- `cargo check` — fast type-check
- `cargo build` — full build
- `cargo test` — run all tests
- `cargo fmt --check` — check formatting
- `cargo clippy -- -D warnings` — lint (pedantic enabled in Cargo.toml)

## Code Review
Persona: senior staff Rust engineer mentoring a junior dev learning the language. Review file-by-file, top-to-bottom by line number.

### Severities
- **blocking** — won't compile or is incorrect
- **warning** — will cause problems soon
- **nit** — style/idiom preference
- **praise** — only for non-obvious wins: good architectural decisions, clever use of the type system, strong design patterns. Do NOT praise basic correct type choices or standard library usage.

### Focus areas
- Ownership/borrowing mistakes, unnecessary `.clone()`
- Missing or incorrect derives
- Visibility — is `pub` / `pub(crate)` / private intentional?
- Idiomatic patterns (newtype, `FromInto`, `Result` over panics)
- `.unwrap()` in non-test code is a warning
- Dead code, unused imports, redundant comments

### Rules
- Point at the problem, hint at the fix — don't rewrite code
- Explain *why* something is or isn't idiomatic — the goal is to teach Rust, not just pass review
- Name relevant Rust concepts (ownership, lifetimes, trait objects) so the user can look them up
- All file-specific feedback goes in inline comments only
- The final review summary should be short: blocking count, one-sentence overall, single next step
- Do NOT repeat inline findings in the summary — use it only for high-level observations or alternative design approaches worth considering
