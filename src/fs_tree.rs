use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
}

impl TreeNode {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        Self {
            name,
            path,
            is_dir,
            children: Vec::new(),
            expanded: false,
        }
    }
}

const SKIP_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    "dist",
    "build",
    ".next",
    ".nuxt",
    ".cache",
    ".venv",
    "__pycache__",
];

fn should_skip(name: &str) -> bool {
    if name.starts_with('.') && name != "." && name != ".." {
        return true;
    }
    SKIP_DIRS.contains(&name)
}

pub fn build_tree(root: &Path) -> anyhow::Result<TreeNode> {
    let mut root_node = TreeNode::new(root.to_path_buf(), true);
    root_node.expanded = true;
    build_tree_recursive(&mut root_node, root)?;
    Ok(root_node)
}

fn build_tree_recursive(node: &mut TreeNode, path: &Path) -> anyhow::Result<()> {
    let mut entries: Vec<_> = std::fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if should_skip(&name_str) {
            continue;
        }
        let entry_path = entry.path();
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let mut child = TreeNode::new(entry_path.clone(), is_dir);
        if is_dir {
            build_tree_recursive(&mut child, &entry_path)?;
        }
        node.children.push(child);
    }
    Ok(())
}
