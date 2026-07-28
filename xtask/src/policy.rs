use crate::fs_walk::files_below;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SHARED_WORKFLOW_REVISION: &str = "624cb41adeed21a6461eb838bc7330bd0a5079fd";
const RETIRED_WORKFLOW_REVISIONS: &[&str] = &[
    "b6caad377102ca73794efaf734a65903b8efa829",
    "79405c457b5b99d5cb9957c9bcdc475109e1e3bf",
];

const REQUIRED_FILES: &[&str] = &[
    ".github/workflows/validation.yml",
    "AGENTS.md",
    "ARCHITECTURE.md",
    "Cargo.toml",
    "Cargo.lock",
    "LICENSE-MIT",
    "LICENSE-APACHE",
    "SECURITY.md",
    "README.md",
    "TESTING.md",
    "docs/architecture.md",
    "docs/provenance/runenwerk-extraction.md",
    "docs/roadmap.md",
    "docs/status-map.md",
    "docs/tooling/validation.md",
    "docs/work-tracking.md",
];

const ALLOWED_MANIFESTS: &[&str] = &[
    "Cargo.toml",
    "conformance/downstream/Cargo.toml",
    "xtask/Cargo.toml",
];

pub fn validate_repository() -> Result<(), String> {
    let root = repository_root()?;
    validate_required_files(&root)?;
    validate_manifest_inventory(&root)?;
    validate_root_manifest(&root)?;
    validate_current_authority(&root)?;
    validate_workflow_inventory(&root)?;
    validate_workflow_authority(&root)?;
    validate_path_dependencies(&root)?;
    validate_source_independence(&root)?;
    validate_no_gitlinks(&root)?;
    validate_provenance(&root)
}

fn repository_root() -> Result<PathBuf, String> {
    std::env::current_dir().map_err(|error| format!("failed to resolve repository root: {error}"))
}

fn validate_required_files(root: &Path) -> Result<(), String> {
    for required in REQUIRED_FILES {
        let path = root.join(required);
        if !path.is_file() {
            return Err(format!("required repository file is missing: {required}"));
        }
        let metadata = fs::metadata(&path)
            .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
        if metadata.len() == 0 {
            return Err(format!("required repository file is empty: {required}"));
        }
    }
    Ok(())
}

fn validate_manifest_inventory(root: &Path) -> Result<(), String> {
    let allowed = ALLOWED_MANIFESTS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    let mut found = BTreeSet::new();

    for path in files_below(root)? {
        if path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml") {
            found.insert(normalized_relative(root, &path)?);
        }
    }

    if found == allowed {
        Ok(())
    } else {
        Err(format!(
            "unexpected Cargo manifest inventory; expected {allowed:?}, found {found:?}"
        ))
    }
}

fn validate_root_manifest(root: &Path) -> Result<(), String> {
    let manifest = read(root.join("Cargo.toml"))?;
    for required in [
        "name = \"runen-sdf\"",
        "rust-version = \"1.93.0\"",
        "license = \"MIT OR Apache-2.0\"",
        "repository = \"https://github.com/dornglut/runen-sdf\"",
        "publish = false",
        "[lints]",
        "workspace = true",
    ] {
        if !manifest.contains(required) {
            return Err(format!(
                "root manifest is missing required declaration: {required}"
            ));
        }
    }
    Ok(())
}

fn validate_current_authority(root: &Path) -> Result<(), String> {
    let checks = [
        (
            "README.md",
            "repository: dornglut/runen-sdf",
            &["repository: Crystonix/runen-sdf"][..],
        ),
        (
            "README.md",
            "[Architecture entrypoint](ARCHITECTURE.md)",
            &[][..],
        ),
        (
            "README.md",
            "[Testing and validation entrypoint](TESTING.md)",
            &[][..],
        ),
        (
            "ARCHITECTURE.md",
            "[Detailed architecture](docs/architecture.md)",
            &["Crystonix/runen-sdf"][..],
        ),
        (
            "TESTING.md",
            "`cargo validate`",
            &["Crystonix/runen-sdf"][..],
        ),
        (
            "AGENTS.md",
            "`ARCHITECTURE.md`",
            &["Crystonix/runen-sdf"][..],
        ),
        ("AGENTS.md", "`TESTING.md`", &[][..]),
        (
            "SECURITY.md",
            "`dornglut/runen-sdf`",
            &["`Crystonix/runen-sdf`"][..],
        ),
        (
            "docs/roadmap.md",
            "`dornglut/runenwerk`",
            &["`Crystonix/runenwerk`"][..],
        ),
        (
            "docs/status-map.md",
            "repository: dornglut/runen-sdf",
            &["repository: Crystonix/runen-sdf"][..],
        ),
        (
            "docs/work-tracking.md",
            "`dornglut/runenwerk`",
            &["`Crystonix/runenwerk`"][..],
        ),
    ];

    for (relative, required, forbidden) in checks {
        let path = root.join(relative);
        let content = read(&path)?;
        if !content.contains(required) {
            return Err(format!(
                "active authority is missing required current identity {required:?}: {relative}"
            ));
        }
        reject_tokens(root, &path, &content, forbidden)?;
    }

    Ok(())
}

fn validate_workflow_inventory(root: &Path) -> Result<(), String> {
    let directory = root.join(".github/workflows");
    let mut found = BTreeSet::new();
    let entries = fs::read_dir(&directory)
        .map_err(|error| format!("failed to read {}: {error}", directory.display()))?;

    for entry in entries {
        let path = entry
            .map_err(|error| format!("failed to read workflow entry: {error}"))?
            .path();
        if !path.is_file()
            || !matches!(
                path.extension().and_then(|extension| extension.to_str()),
                Some("yml" | "yaml")
            )
        {
            continue;
        }
        found.insert(normalized_relative(root, &path)?);
    }

    let expected = BTreeSet::from([".github/workflows/validation.yml".to_owned()]);
    if found == expected {
        Ok(())
    } else {
        Err(format!(
            "unexpected workflow inventory; expected {expected:?}, found {found:?}"
        ))
    }
}

fn workflow_blocks(workflow: &str) -> Result<Vec<(String, Vec<String>)>, String> {
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();

    for raw_line in workflow.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            let Some((_, contents)) = blocks.last_mut() else {
                return Err(
                    "validation workflow has indented content before a top-level key".to_owned(),
                );
            };
            contents.push(line.to_owned());
            continue;
        }

        let Some((key, value)) = line.split_once(':') else {
            return Err("validation workflow has malformed top-level content".to_owned());
        };
        if key.is_empty()
            || !key.chars().all(|character| {
                character.is_ascii_alphanumeric() || character == '_' || character == '-'
            })
            || (!value.is_empty() && !value.starts_with(' '))
        {
            return Err("validation workflow has malformed top-level content".to_owned());
        }
        blocks.push((key.to_owned(), vec![line.to_owned()]));
    }

    Ok(blocks)
}

fn workflow_block<'a>(blocks: &'a [(String, Vec<String>)], key: &str) -> Option<&'a [String]> {
    blocks
        .iter()
        .find(|(block_key, _)| block_key == key)
        .map(|(_, lines)| lines.as_slice())
}

fn matches_workflow_block(block: Option<&[String]>, expected: &[&str]) -> bool {
    matches!(block, Some(lines) if lines.iter().map(String::as_str).eq(expected.iter().copied()))
}

fn validate_workflow_authority(root: &Path) -> Result<(), String> {
    let path = root.join(".github/workflows/validation.yml");
    let workflow = read(&path)?;
    let blocks = workflow_blocks(&workflow)?;
    let keys = blocks
        .iter()
        .map(|(key, _)| key.as_str())
        .collect::<Vec<_>>();
    let expected_keys = ["name", "on", "permissions", "concurrency", "jobs"];
    if keys != expected_keys {
        return Err("validation workflow must have exactly name, on, permissions, concurrency, and jobs top-level blocks".to_owned());
    }
    if keys.iter().collect::<BTreeSet<_>>().len() != keys.len() {
        return Err("validation workflow must not duplicate top-level keys".to_owned());
    }

    if !matches_workflow_block(
        workflow_block(&blocks, "name"),
        &["name: RunenSDF Validation"],
    ) {
        return Err("validation workflow identity must be RunenSDF Validation".to_owned());
    }
    if !matches_workflow_block(
        workflow_block(&blocks, "on"),
        &[
            "on:",
            "  pull_request:",
            "    branches:",
            "      - main",
            "  push:",
            "    branches:",
            "      - main",
            "  workflow_dispatch:",
        ],
    ) {
        return Err("validation workflow triggers must be the accepted main pull-request, main push, and unconfigured dispatch set".to_owned());
    }
    if !matches_workflow_block(
        workflow_block(&blocks, "permissions"),
        &["permissions:", "  contents: read"],
    ) {
        return Err("validation workflow permissions must be exactly contents: read".to_owned());
    }
    if !matches_workflow_block(
        workflow_block(&blocks, "concurrency"),
        &[
            "concurrency:",
            "  group: runen-sdf-validation-${{ github.workflow }}-${{ github.ref }}",
            "  cancel-in-progress: true",
        ],
    ) {
        return Err(
            "validation workflow concurrency must match the accepted group and cancellation policy"
                .to_owned(),
        );
    }

    let expected_uses = format!(
        "    uses: dornglut/github-workflows/.github/workflows/reusable-rust-cargo-validate.yml@{SHARED_WORKFLOW_REVISION}"
    );
    let expected_job = [
        "jobs:",
        "  validate:",
        "    name: Validate standalone framework",
        expected_uses.as_str(),
    ];
    if !matches_workflow_block(workflow_block(&blocks, "jobs"), &expected_job) {
        return Err(
            "validation workflow must contain only the accepted validate reusable job".to_owned(),
        );
    }

    for revision in RETIRED_WORKFLOW_REVISIONS {
        if workflow.contains(revision) {
            return Err(format!(
                "validation workflow contains retired reusable workflow revision: {revision}"
            ));
        }
    }

    Ok(())
}

fn validate_path_dependencies(root: &Path) -> Result<(), String> {
    for manifest in ALLOWED_MANIFESTS {
        let manifest_path = root.join(manifest);
        let content = read(&manifest_path)?;
        for line in content.lines() {
            let Some(path_value) = quoted_assignment(line, "path") else {
                continue;
            };
            let parent = manifest_path.parent().ok_or_else(|| {
                format!(
                    "manifest has no parent directory: {}",
                    manifest_path.display()
                )
            })?;
            let joined = parent.join(path_value);
            let canonical = joined.canonicalize().map_err(|error| {
                format!("invalid path dependency {}: {error}", joined.display())
            })?;
            if !canonical.starts_with(root) {
                return Err(format!(
                    "path dependency escapes repository: {}",
                    canonical.display()
                ));
            }
        }
    }
    Ok(())
}

fn validate_source_independence(root: &Path) -> Result<(), String> {
    for manifest in ALLOWED_MANIFESTS {
        let path = root.join(manifest);
        let content = read(&path)?;
        reject_tokens(
            root,
            &path,
            &content,
            &["name = \"sdf\"", "package = \"sdf\"", "runenwerk"],
        )?;
    }

    for path in files_below(root)? {
        let relative = normalized_relative(root, &path)?;
        let is_public_rust = relative.starts_with("src/")
            || relative.starts_with("tests/")
            || relative.starts_with("conformance/downstream/src/");
        if !is_public_rust || path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }

        let content = read(&path)?;
        reject_tokens(
            root,
            &path,
            &content,
            &[
                "use sdf::",
                "extern crate sdf",
                "include!",
                "#[path",
                "runenwerk",
            ],
        )?;
    }

    for forbidden_directory in ["crates", "domain"] {
        if root.join(forbidden_directory).exists() {
            return Err(format!(
                "forbidden repository directory exists: {forbidden_directory}"
            ));
        }
    }

    let lockfile = read(root.join("Cargo.lock"))?;
    if lockfile
        .to_ascii_lowercase()
        .contains("name = \"runenwerk\"")
    {
        return Err("Cargo.lock contains a Runenwerk package".to_owned());
    }

    Ok(())
}

fn reject_tokens(
    root: &Path,
    path: &Path,
    content: &str,
    forbidden: &[&str],
) -> Result<(), String> {
    let lowercase = content.to_ascii_lowercase();
    for token in forbidden {
        if lowercase.contains(&token.to_ascii_lowercase()) {
            return Err(format!(
                "forbidden token {token:?} in {}",
                normalized_relative(root, path)?
            ));
        }
    }
    Ok(())
}

fn validate_no_gitlinks(root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["ls-files", "-s"])
        .output()
        .map_err(|error| format!("failed to inspect git index: {error}"))?;
    if !output.status.success() {
        return Err("git ls-files -s failed".to_owned());
    }
    let index = String::from_utf8_lossy(&output.stdout);
    if index.lines().any(|line| line.starts_with("160000 ")) {
        Err("git submodules are forbidden".to_owned())
    } else {
        Ok(())
    }
}

fn validate_provenance(root: &Path) -> Result<(), String> {
    let provenance = read(root.join("docs/provenance/runenwerk-extraction.md"))?;
    for required in [
        "dornglut/runen-sdf",
        "dornglut/runenwerk",
        "d52badefc640d6dc6dcdd40268af3aea1bb8eefe",
        "8de096259eab30f8d67672010df9190970d0bfc4",
        "domain/sdf",
        "PT-RUNENSDF-003",
    ] {
        if !provenance.contains(required) {
            return Err(format!(
                "provenance is missing required authority: {required}"
            ));
        }
    }
    Ok(())
}

fn quoted_assignment<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let marker = format!("{key} = \"");
    let start = line.find(&marker)? + marker.len();
    let remainder = &line[start..];
    let end = remainder.find('"')?;
    Some(&remainder[..end])
}

fn read(path: impl AsRef<Path>) -> Result<String, String> {
    let path = path.as_ref();
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn normalized_relative(root: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(root)
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
        .map_err(|error| format!("failed to relativize {}: {error}", path.display()))
}
