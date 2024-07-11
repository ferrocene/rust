use anyhow::Error;
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};

mod cargo_metadata;

/// The entry point to the binary.
///
/// You should probably let `bootstrap` execute this program instead of running it directly.
///
/// Run `x.py run generate-metadata`
fn main() -> Result<(), Error> {
    let dest_file = env_path("DEST")?;
    let out_dir = env_path("OUT_DIR")?;
    let cargo = env_path("CARGO")?;
    let license_metadata = env_path("LICENSE_METADATA")?;

    let collected_tree_metadata: Metadata =
        serde_json::from_slice(&std::fs::read(&license_metadata)?)?;

    let root_path = std::path::absolute(".")?;
    let workspace_paths = [
        Path::new("./Cargo.toml"),
        Path::new("./src/tools/cargo/Cargo.toml"),
        Path::new("./library/std/Cargo.toml"),
    ];
    let collected_cargo_metadata =
        cargo_metadata::get(&cargo, &out_dir, &root_path, &workspace_paths)?;

    let mut license_set = BTreeSet::new();

    let mut buffer = Vec::new();

    writeln!(buffer, "# COPYRIGHT for Rust")?;
    writeln!(buffer)?;
    writeln!(
        buffer,
        "This file describes the copyright and licensing information for the source code within The Rust Project git tree, and the third-party dependencies used when building the Rust toolchain (including the Rust Standard Library)"
    )?;
    writeln!(buffer)?;
    writeln!(buffer, "## Table of Contents")?;
    writeln!(buffer)?;
    writeln!(buffer, "* [In-tree files](#in-tree-files)")?;
    writeln!(buffer, "* [Out-of-tree files](#out-of-tree-files)")?;
    // writeln!(buffer, "* [License Texts](#license-texts)")?;
    writeln!(buffer)?;

    writeln!(buffer, "## In-tree files")?;
    writeln!(buffer)?;
    writeln!(
        buffer,
        "The following licenses cover the in-tree source files that were used in this release:"
    )?;
    writeln!(buffer)?;
    render_tree_recursive(&collected_tree_metadata.files, &mut buffer, 0, &mut license_set)?;

    writeln!(buffer)?;

    writeln!(buffer, "## Out-of-tree files")?;
    writeln!(buffer)?;
    writeln!(
        buffer,
        "The following licenses cover the out-of-tree crates that were used in this release:"
    )?;
    writeln!(buffer)?;
    render_deps(collected_cargo_metadata.iter(), &mut buffer, &mut license_set)?;

    std::fs::write(&dest_file, &buffer)?;

    Ok(())
}

/// Recursively draw the tree of files/folders we found on disk and their licenses, as
/// markdown, into the given Vec.
fn render_tree_recursive(
    node: &Node,
    buffer: &mut Vec<u8>,
    depth: usize,
    license_set: &mut BTreeSet<String>,
) -> Result<(), Error> {
    let prefix = std::iter::repeat("> ").take(depth + 1).collect::<String>();

    match node {
        Node::Root { children } => {
            for child in children {
                render_tree_recursive(child, buffer, depth, license_set)?;
            }
        }
        Node::Directory { name, children, license } => {
            render_tree_license(
                &prefix,
                std::iter::once(name),
                license.iter(),
                buffer,
                license_set,
            )?;
            if !children.is_empty() {
                writeln!(buffer, "{prefix}")?;
                writeln!(buffer, "{prefix}*Exceptions:*")?;
                for child in children {
                    writeln!(buffer, "{prefix}")?;
                    render_tree_recursive(child, buffer, depth + 1, license_set)?;
                }
            }
        }
        Node::CondensedDirectory { name, licenses } => {
            render_tree_license(
                &prefix,
                std::iter::once(name),
                licenses.iter(),
                buffer,
                license_set,
            )?;
        }
        Node::Group { files, directories, license } => {
            render_tree_license(
                &prefix,
                directories.iter().chain(files.iter()),
                std::iter::once(license),
                buffer,
                license_set,
            )?;
        }
        Node::File { name, license } => {
            render_tree_license(
                &prefix,
                std::iter::once(name),
                std::iter::once(license),
                buffer,
                license_set,
            )?;
        }
    }

    Ok(())
}

/// Draw a series of sibling files/folders, as markdown, into the given Vec.
fn render_tree_license<'a>(
    prefix: &str,
    names: impl Iterator<Item = &'a String>,
    licenses: impl Iterator<Item = &'a License>,
    buffer: &mut Vec<u8>,
    license_set: &mut BTreeSet<String>,
) -> Result<(), Error> {
    let mut spdxs = BTreeSet::new();
    let mut copyrights = BTreeSet::new();
    for license in licenses {
        spdxs.insert(&license.spdx);
        license_set.insert(license.spdx.clone());
        for copyright in &license.copyright {
            copyrights.insert(copyright);
        }
    }

    for name in names {
        writeln!(buffer, "{prefix}**`{name}`**  ")?;
    }
    for spdx in spdxs.iter() {
        writeln!(buffer, "{prefix}License: `{spdx}`  ")?;
    }
    for (i, copyright) in copyrights.iter().enumerate() {
        let suffix = if i == copyrights.len() - 1 { "" } else { "  " };
        writeln!(buffer, "{prefix}Copyright: {copyright}{suffix}")?;
    }

    Ok(())
}

/// Render a list of out-of-tree dependencies as markdown into the given Vec.
fn render_deps<'a, 'b>(
    deps: impl Iterator<Item = &'a cargo_metadata::Dependency>,
    buffer: &'b mut Vec<u8>,
    license_set: &mut BTreeSet<String>,
) -> Result<(), Error> {
    for dep in deps {
        let authors_list = dep.authors.join(", ").replace("<", "\\<").replace(">", "\\>");
        let url = format!("https://crates.io/crates/{}/{}", dep.name, dep.version);
        writeln!(buffer)?;
        writeln!(
            buffer,
            "### [{name} {version}]({url})",
            name = dep.name,
            version = dep.version,
            url = url,
        )?;
        writeln!(buffer)?;
        writeln!(buffer, "* Authors: {}", authors_list)?;
        writeln!(buffer, "* License: {}", dep.license)?;
        license_set.insert(dep.license.clone());
        for (name, contents) in &dep.notices {
            writeln!(buffer)?;
            writeln!(buffer, "#### {}", name.to_string_lossy())?;
            writeln!(buffer)?;
            writeln!(buffer, "<details><summary>Click to expand</summary>")?;
            writeln!(buffer)?;
            writeln!(buffer, "```")?;
            writeln!(buffer, "{}", contents)?;
            writeln!(buffer, "```")?;
            writeln!(buffer)?;
            writeln!(buffer, "</details>")?;
        }
    }
    Ok(())
}
/// Describes a tree of metadata for our filesystem tree
#[derive(serde::Deserialize)]
struct Metadata {
    files: Node,
}

/// Describes one node in our metadata tree
#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub(crate) enum Node {
    Root { children: Vec<Node> },
    Directory { name: String, children: Vec<Node>, license: Option<License> },
    CondensedDirectory { name: String, licenses: Vec<License> },
    File { name: String, license: License },
    Group { files: Vec<String>, directories: Vec<String>, license: License },
}

/// A License has an SPDX license name and a list of copyright holders.
#[derive(serde::Deserialize)]
struct License {
    spdx: String,
    copyright: Vec<String>,
}

/// Grab an environment variable as a PathBuf, or fail nicely.
fn env_path(var: &str) -> Result<PathBuf, Error> {
    if let Some(var) = std::env::var_os(var) {
        Ok(var.into())
    } else {
        anyhow::bail!("missing environment variable {var}")
    }
}
