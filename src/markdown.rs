use crate::language::extension_to_language;
use std::path::Path;

pub fn path_to_markdown_path(root: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let mut result = String::from("@/");
    for component in relative.components() {
        if let std::path::Component::Normal(os_str) = component {
            if let Some(s) = os_str.to_str() {
                result.push_str(s);
                result.push('/');
            }
        }
    }
    if result.ends_with('/') {
        result.pop();
    }
    result.push(':');
    result
}

pub fn generate_markdown_entry(root: &Path, path: &Path, content: &str, is_diff: bool) -> String {
    let markdown_path = path_to_markdown_path(root, path);

    let language = if is_diff {
        "diff"
    } else {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        extension_to_language(ext)
    };

    let mut fence_len = 3;
    let mut fence = "`".repeat(fence_len);
    while content.contains(&fence) {
        fence_len += 1;
        fence = "`".repeat(fence_len);
    }

    let mut entry = format!("## {}\n{}{}\n{}", markdown_path, fence, language, content);
    if !entry.ends_with('\n') {
        entry.push('\n');
    }
    entry.push_str(&fence);
    entry.push_str("\n\n");
    entry
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_path_to_markdown_path() {
        let root = PathBuf::from("/project");
        let file = PathBuf::from("/project/src/main.rs");
        let md_path = path_to_markdown_path(&root, &file);
        assert_eq!(md_path, "@/src/main.rs:");
    }

    #[test]
    fn test_dynamic_fences() {
        let root = PathBuf::from("/project");
        let file = PathBuf::from("/project/README.md");
        let content = "Here is some code:\n```rust\nfn main() {}\n```";
        let entry = generate_markdown_entry(&root, &file, content, false);

        assert!(entry.contains("````markdown\nHere is some code"));
        assert!(entry.ends_with("````\n\n"));
    }
}
