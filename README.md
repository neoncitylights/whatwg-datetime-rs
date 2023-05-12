# Rust Repository Template 🦀

Repository template to get quickly started with writing Rust libraries, ready for distributing.

## Getting started

Open your favorite terminal and clone this locally.

- With the [GitHub CLI](https://cli.github.com/) (replace `<project>` with what you'd like to call your project):

   ```shell
   gh repo create <project> --template nlp-rs/rust-template
   ```

- With the Git CLI:

   ```shell
   git clone https://github.com/nlp-rs/rust-template.git
   ```

## Features

- [x] Remote development support with [GitHub Codespaces](https://github.com/features/codespaces)
- [x] Debugging with the [LLDB Debugger](https://lldb.llvm.org/) tool in Visual Studio Code ([VSCode Marketplace](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb))
- [x] Fuzz testing with LLVM's [libFuzzer](https://llvm.org/docs/LibFuzzer.html) tool and [`cargo fuzz`](https://github.com/rust-fuzz/cargo-fuzz) ([Reference](https://rust-fuzz.github.io/book/introduction.html))
- [x] Performance benchmarks in Rust with Criterion and Iai (suitable to run in GitHub Actions CI environments)
- [x] CI/CD support with [GitHub Actions](https://github.com/features/actions)
- [x] Running tests and benchmarks
- [x] Running [Rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) for detecting formatting and linting errors, respectively
- [x] Weekly, midnight scheduled audits of Rust packages (for outdated dependencies, compatible software licenses, and software vulnerabilities) with [`EmbarkStudios/cargo-deny-action`](https://github.com/EmbarkStudios/cargo-deny-action)

## Configure

| Tool                     | File path                                                | Reference                                                                                                        |
|--------------------------|----------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| GitHub Codespaces        | [`devcontainer.json`](./.devcontainer/devcontainer.json) | [Reference](https://containers.dev/implementors/json_reference/)                                                 |
| GitHub Actions           | [`.github/workflows`](./.github/workflows)               | [Reference](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions)               |
| Cargo package            | [`Cargo.toml`](./Cargo.toml)                            | [Reference](https://doc.rust-lang.org/cargo/reference/manifest.html)                                             |
| Clippy (Rust linter)     | [`.clippy.toml`](./.clippy.toml)                         | [Repository](https://github.com/rust-lang/rust-clippy), [Reference]( https://rust-lang.github.io/rust-clippy/) |
| Rustfmt (Rust formatter) | [`.rustfmt.toml`](./.rustfmt.toml)                       | [Repository](https://github.com/rust-lang/rustfmt), [Reference](https://rust-lang.github.io/rustfmt/)           |
| Commitlint               | [`.commitlintrc.json`](./.commitlintrc.json)            | [Repository](https://github.com/conventional-changelog/commitlint), [Reference](https://commitlint.js.org/#/)    |
| `cargo-deny`             | [`deny.toml`](./deny.toml)                               | [Repository](https://github.com/EmbarkStudios/cargo-deny)                                                        |

## Run scripts locally

| Script      | Command |
|-------------|---------|
| Run unit/integration/doc tests | `cargo test` |
| Run fuzz tests | `cargo fuzz <fuzz-target>` |
| Run Rustfmt | `cargo fmt` |
| Run Clippy | `cargo clippy` |
| Run performance benchmarks | `cargo bench` |
| Generate API docs for crate | `cargo doc` |
| Generate mdBook docs for crate | `mdbook build` |
| Run security audits | `cargo audit`[^cargo-audit] |

[^cargo-audit]: Requires installing [`cargo-audit`](https://crates.io/crates/cargo-audit) locally
