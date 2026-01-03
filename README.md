# Advent of Code 2025

This repo contains my **Advent of Code 2025** solutions, written in **Rust**.

Nothing fancy here just clean, readable solutions. Each day is split into **Part A** and **Part B**, each as its own small Rust binary, with a shared utils crate and a generator to keep things tidy.

---

## Progress

| Day | Part A | Part B |
| --- | ------ | ------ |
| 01  | ✅      | ✅      |
| 02  | ✅      | ⬜      |
| 03  | ⬜      | ⬜      |
| 04  | ⬜      | ⬜      |
| 05  | ⬜      | ⬜      |
| 06  | ⬜      | ⬜      |
| 07  | ⬜      | ⬜      |
| 08  | ⬜      | ⬜      |
| 09  | ⬜      | ⬜      |
| 10  | ⬜      | ⬜      |
| 11  | ⬜      | ⬜      |
| 12  | ⬜      | ⬜      |

(⬜ = not done, ✅ = done)

---

## Repository structure

```text
.
├── Cargo.toml                # Workspace definition
├── aoc_utils/                # Shared helpers (input loading, utils, etc.)
│   └── src/lib.rs
├── scripts/
│   └── scaffold_day/         # Rust-based generator for new days
│       └── src/main.rs
├── day01/
│   ├── part_a/
│   │   └── src/main.rs
│   └── part_b/
│       └── src/main.rs
├── inputs/
│   └── day01/
│       ├── input_a.txt
│       └── input_b.txt
└── ...
```

---

## Getting started

### Prerequisites

- Rust (stable)

```bash
rustc --version
cargo --version
```

### Clone the repo

```bash
git clone https://github.com/sger/advent-of-code-2025.git
cd advent-of-code-2025
```

---

## Scaffolding a new day

Generate Day 1 (or any day number):

```bash
cargo run -p scaffold_day -- 1
```

This will:

- Create:
  - `day01/part_a`
  - `day01/part_b`
  - `inputs/day01/input_a.txt`
  - `inputs/day01/input_b.txt`
- Regenerate the workspace members **safely**
- Avoid referencing missing crates

---

## Running solutions

### Part A

```bash
cargo run -p day01_part_a
```

### Part B

```bash
cargo run -p day01_part_b
```

---

## Workspace safety (important)

Cargo fails if **any workspace member path is missing**.

To avoid this:

- The workspace only permanently includes:
  - `aoc_utils`
  - `scripts/scaffold_day`
- Day crates are **added automatically** and **removed automatically** by the scaffold tool based on what exists on disk.

If you delete a `dayNN/` folder, just run the scaffold tool again to fix the workspace.

---

## Development workflow

Typical daily flow:

```bash
cargo run -p scaffold_day -- 7
# paste puzzle input into inputs/day07/input_a.txt
# paste puzzle input into inputs/day07/input_b.txt
cargo run -p day07_part_a
cargo run -p day07_part_b
```

---

## License

This project is released under the **GNU GPL-3.0** license.

Check out the [LICENSE](LICENSE) file for more information.

[aoc-2025]: https://adventofcode.com/2025
