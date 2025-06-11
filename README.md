A project to learn Rust while covering its core concepts. 
Rust is a systems programming language that emphasizes safety and performance.


a **multi-component project** that gradually introduces and applies key Rust concepts.

Rust Project**: "RustyBox" – A Modular Command-Line Toolbox

Concept:

a CLI (Command-Line Interface) app called `rustybox` that provides a suite of small utilities/tools like:

* File explorer
* Word counter
* Task manager (todo list)
* Note keeper (save/load/delete notes)
* Math expression evaluator
* JSON/YAML data viewer
* Web scraper
* REST API client

Each subtool introduces specific Rust features.

Curriculum Roadmap with Concepts

| Stage               | Feature                             | Rust Concepts Covered                           |
| ------------------- | ----------------------------------- | ----------------------------------------------- |
| 1. Setup            | Project structure, CLI parsing      | `cargo`, `clap`, `mod`, `main`, `args`, `match` |
| 2. File Explorer    | File I/O, pattern matching          | `std::fs`, `Path`, `Result`, `Option`           |
| 3. Word Counter     | Error handling, string manipulation | `String`, `Vec`, `Result`, `unwrap`, `match`    |
| 4. Task Manager     | Structs, enums, basic persistence   | `struct`, `enum`, `serde`, `serde_json`         |
| 5. Note Keeper      | Lifetimes, ownership, borrowing     | References, slices, lifetimes                   |
| 6. Math Evaluator   | Expressions, error handling, traits | `trait`, `impl`, error types                    |
| 7. JSON/YAML Viewer | External crates, serde, generics    | `serde_json`, `serde_yaml`, generics            |
| 8. Web Scraper      | Async, HTTP, external crates        | `reqwest`, `tokio`, `futures`                   |
| 9. API Client       | REST, structs, async                | `serde`, `reqwest`, JSON APIs                   |


Project Structure

rustybox/
│
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs          # CLI parsing
│   ├── file_explorer.rs
│   ├── word_counter.rs
│   ├── task_manager.rs
│   ├── note_keeper.rs
│   ├── math_eval.rs
│   ├── json_viewer.rs
│   ├── web_scraper.rs
│   └── api_client.rs


Tools and Crates

[`clap`](https://crates.io/crates/clap) – CLI parsing
[`serde`, `serde_json`, `serde_yaml`](https://serde.rs) – Serialization
[`reqwest`](https://crates.io/crates/reqwest) – HTTP client
[`tokio`](https://crates.io/crates/tokio) – Async runtime
[`regex`](https://crates.io/crates/regex) – Regular expressions
[`thiserror`](https://crates.io/crates/thiserror) – Error handling

