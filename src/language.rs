pub fn extension_to_language(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "tsx" => "tsx",
        "ts" | "mts" | "cts" => "typescript",
        "jsx" => "jsx",
        "js" | "mjs" | "cjs" => "javascript",
        "rs" => "rust",
        "py" => "python",
        "json" => "json",
        "jsonc" => "jsonc",
        "toml" => "toml",
        "yaml" | "yml" => "yaml",
        "md" => "markdown",
        "html" => "html",
        "css" => "css",
        "scss" => "scss",
        "go" => "go",
        "c" | "h" => "c",
        "cpp" | "cc" | "hpp" => "cpp",
        "sh" | "bash" => "bash",
        "ps1" => "powershell",
        "sql" => "sql",
        "txt" => "text",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_to_language() {
        assert_eq!(extension_to_language("rs"), "rust");
        assert_eq!(extension_to_language("TSX"), "tsx");
        assert_eq!(extension_to_language("unknown"), "");
        assert_eq!(extension_to_language("yml"), "yaml");
    }
}
