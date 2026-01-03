use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn main() -> anyhow::Result<()> {
    let day = parse_day_arg()?;
    let day_padded = format!("{:02}", day);

    let root_dir = find_repo_root()?;
    let root_cargo = root_dir.join("Cargo.toml");

    println!("Using repo root: {}", root_dir.display());
    println!("Using Cargo.toml: {}", root_cargo.display());

    // Inputs
    let inputs_day_dir = root_dir.join("inputs").join(format!("day{day_padded}"));
    ensure_file(
        &inputs_day_dir.join("input_a.txt"),
        format!("# Paste your input for day {day_padded} part A here.\n"),
    )?;
    ensure_file(
        &inputs_day_dir.join("input_b.txt"),
        format!("# Paste your input for day {day_padded} part B here.\n"),
    )?;

    // Crates
    let day_dir = root_dir.join(format!("day{day_padded}"));
    let part_a_dir = day_dir.join("part_a");
    let part_b_dir = day_dir.join("part_b");

    create_crate(
        &part_a_dir,
        &format!("day{day_padded}_part_a"),
        &format!("../inputs/day{day_padded}/input_a.txt"),
    )?;
    create_crate(
        &part_b_dir,
        &format!("day{day_padded}_part_b"),
        &format!("../inputs/day{day_padded}/input_b.txt"),
    )?;

    // Workspace update
    ensure_workspace_member(&root_cargo, &format!("day{day_padded}/part_a"))?;
    ensure_workspace_member(&root_cargo, &format!("day{day_padded}/part_b"))?;

    println!("Scaffolded day {day_padded}");
    println!("Run:");
    println!("  cargo run -p day{day_padded}_part_a");
    println!("  cargo run -p day{day_padded}_part_b");

    Ok(())
}

fn parse_day_arg() -> anyhow::Result<u32> {
    let mut args = env::args().skip(1);
    let day_str = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("Usage: scaffold_day <day_number>"))?;

    let day: u32 = day_str
        .parse()
        .map_err(|_| anyhow::anyhow!("Day must be a number (>= 1)"))?;

    if day == 0 {
        return Err(anyhow::anyhow!("Day must be >= 1"));
    }

    Ok(day)
}

/// Find repo root by walking up until we find Cargo.toml with a [workspace] header.
/// This makes the tool resilient to being launched from subdirs.
fn find_repo_root() -> anyhow::Result<PathBuf> {
    let mut dir = env::current_dir()?;
    loop {
        let candidate = dir.join("Cargo.toml");
        if candidate.exists() {
            let text = fs::read_to_string(&candidate)?;
            if text.contains("[workspace]") {
                return Ok(dir);
            }
        }
        if !dir.pop() {
            break;
        }
    }
    Err(anyhow::anyhow!(
        "Could not find repo root (Cargo.toml with [workspace])"
    ))
}

fn ensure_file(path: &Path, content: String) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}

fn create_crate(crate_dir: &Path, crate_name: &str, rel_input: &str) -> io::Result<()> {
    let cargo_toml = crate_dir.join("Cargo.toml");
    if cargo_toml.exists() {
        // Don't overwrite existing crates.
        return Ok(());
    }

    fs::create_dir_all(crate_dir.join("src"))?;

    // From dayNN/part_* to repo root aoc_utils is ../../aoc_utils
    let cargo_contents = format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
aoc_utils = {{ path = "../../aoc_utils" }}
"#
    );
    fs::write(&cargo_toml, cargo_contents)?;

    let main_rs = format!(
        r#"use aoc_utils::read_input;

fn main() {{
    let input = read_input("{rel_input}");
    let answer = solve(&input);
    println!("{{answer}}");
}}

fn solve(input: &str) -> String {{
    let _ = input.trim_end();
    "TODO".to_string()
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn smoke_test() {{
        assert_eq!(solve(""), "TODO");
    }}
}}
"#
    );
    fs::write(crate_dir.join("src").join("main.rs"), main_rs)?;

    Ok(())
}

/// Workspace update:
/// - If member already exists anywhere as a quoted string, do nothing.
/// - Else, insert it after the line `members = [` if present.
/// - If no members block, create one under [workspace].
fn ensure_workspace_member(root_cargo: &Path, member: &str) -> anyhow::Result<()> {
    let mut text = fs::read_to_string(root_cargo)?;

    if text.contains(&format!("\"{member}\"")) {
        return Ok(());
    }

    // Ensure [workspace] exists
    if !text.contains("[workspace]") {
        text = format!("[workspace]\nmembers = [\n  \"{member}\",\n]\nresolver = \"2\"\n");
        fs::write(root_cargo, text)?;
        return Ok(());
    }

    // Try to find 'members = [' line
    if let Some(idx) = text.find("members = [") {
        // Insert after that line (end of line)
        let line_end = text[idx..]
            .find('\n')
            .map(|o| idx + o + 1)
            .unwrap_or(text.len());
        text.insert_str(line_end, &format!("  \"{member}\",\n"));
        fs::write(root_cargo, text)?;
        return Ok(());
    }

    // If workspace exists but no members, inject right after [workspace] line
    if let Some(ws_idx) = text.find("[workspace]") {
        let ws_line_end = text[ws_idx..]
            .find('\n')
            .map(|o| ws_idx + o + 1)
            .unwrap_or(text.len());
        let injection = format!("members = [\n  \"{member}\",\n]\n");
        text.insert_str(ws_line_end, &injection);
        fs::write(root_cargo, text)?;
        return Ok(());
    }

    Ok(())
}
