use std::collections::HashMap;
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
            children: Vec::new(),
            is_dir,
            expanded: false,
        }
    }
}

const SKIP_DIRS: &[&str] = &[
    ".git",
    // "target",
    // "node_modules",
    // "dist",
    // "build",
    // ".next",
    // ".nuxt",
    // ".cache",
    // ".venv",
    // "__pycache__",
];

pub fn build_tree(root: &Path, filter_ignored: bool) -> anyhow::Result<TreeNode> {
    let mut root_node = TreeNode::new(root.to_path_buf(), true);
    root_node.expanded = true;

    let walker = ignore::WalkBuilder::new(root)
        .hidden(filter_ignored)
        .git_ignore(filter_ignored)
        .git_global(filter_ignored)
        .git_exclude(filter_ignored)
        .ignore(filter_ignored)
        .parents(filter_ignored)
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            let name = e.file_name().to_string_lossy();
            !SKIP_DIRS.contains(&name.as_ref())
        })
        .build();

    let mut map: HashMap<PathBuf, Vec<(PathBuf, bool)>> = HashMap::new();
    for entry in walker.filter_map(|e| e.ok()) {
        let depth = entry.depth();
        if depth == 0 {
            continue;
        }
        let path = entry.path().to_path_buf();
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if let Some(parent) = path.parent() {
            map.entry(parent.to_path_buf())
                .or_default()
                .push((path, is_dir));
        }
    }

    build_from_map(&mut root_node, &map);
    sort_children(&mut root_node);
    Ok(root_node)
}

fn build_from_map(node: &mut TreeNode, map: &HashMap<PathBuf, Vec<(PathBuf, bool)>>) {
    let Some(entries) = map.get(&node.path) else {
        return;
    };
    for (path, is_dir) in entries {
        let mut child = TreeNode::new(path.clone(), *is_dir);
        if *is_dir {
            build_from_map(&mut child, map);
        }
        node.children.push(child);
    }
}

fn sort_children(node: &mut TreeNode) {
    node.children.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    for child in &mut node.children {
        sort_children(child);
    }
}

pub fn collect_files(node: &TreeNode, out: &mut Vec<PathBuf>) {
    if node.is_dir {
        for c in &node.children {
            collect_files(c, out);
        }
    } else {
        out.push(node.path.clone());
    }
}

pub fn collect_dirs(node: &TreeNode, out: &mut Vec<PathBuf>) {
    if node.is_dir {
        out.push(node.path.clone());
        for c in &node.children {
            collect_dirs(c, out);
        }
    }
}
