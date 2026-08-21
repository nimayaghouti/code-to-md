use super::{AppState, is_binary_bytes};

impl AppState {
    pub fn generate_markdown(&mut self) {
        let Some(ref root) = self.project_root.clone() else {
            return;
        };
        if self.selected_files.is_empty() {
            self.status = "No files selected for export".to_string();
            return;
        }

        let output_path = if let Some(ref path) = self.output_path {
            path.clone()
        } else if let Some(path) = self.select_output_path_dialog() {
            path
        } else {
            self.status = "Generate canceled: no output path chosen".to_string();
            return;
        };

        let mut output_text = String::new();
        let mut written_count = 0;
        let mut skipped_count = 0;

        for path in &self.selected_files {
            if !path.exists() || !path.is_file() {
                skipped_count += 1;
                continue;
            }

            match std::fs::read(path) {
                Ok(bytes) => {
                    if is_binary_bytes(&bytes) {
                        skipped_count += 1;
                        continue;
                    }
                    match String::from_utf8(bytes) {
                        Ok(content) => {
                            let entry =
                                crate::markdown::generate_markdown_entry(root, path, &content);
                            output_text.push_str(&entry);
                            written_count += 1;
                        }
                        Err(_) => skipped_count += 1,
                    }
                }
                Err(_) => skipped_count += 1,
            }
        }

        match std::fs::write(&output_path, output_text) {
            Ok(_) => {
                self.status = format!(
                    "Generated {} file(s) into {}. Skipped {} file(s).",
                    written_count,
                    output_path.display(),
                    skipped_count
                );
            }
            Err(e) => {
                self.status = format!("Error writing output file: {}", e);
            }
        }
    }

    pub fn append_markdown(&mut self) {
        let Some(ref root) = self.project_root.clone() else {
            return;
        };
        if self.selected_files.is_empty() {
            self.status = "No files selected for export".to_string();
            return;
        }

        let output_path = if let Some(ref path) = self.output_path {
            path.clone()
        } else if let Some(path) = self.select_output_path_dialog() {
            path
        } else {
            self.status = "Append canceled: no output path chosen".to_string();
            return;
        };

        let mut output_text = String::new();
        let mut written_count = 0;
        let mut skipped_count = 0;

        for path in &self.selected_files {
            if !path.exists() || !path.is_file() {
                skipped_count += 1;
                continue;
            }

            match std::fs::read(path) {
                Ok(bytes) => {
                    if is_binary_bytes(&bytes) {
                        skipped_count += 1;
                        continue;
                    }
                    match String::from_utf8(bytes) {
                        Ok(content) => {
                            let entry =
                                crate::markdown::generate_markdown_entry(root, path, &content);
                            output_text.push_str(&entry);
                            written_count += 1;
                        }
                        Err(_) => skipped_count += 1,
                    }
                }
                Err(_) => skipped_count += 1,
            }
        }

        let mut prefix = String::new();
        if output_path.exists() {
            if let Ok(existing) = std::fs::read_to_string(&output_path) {
                if !existing.is_empty() && !existing.ends_with('\n') {
                    prefix.push('\n');
                }
            }
        }

        let mut file = match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_path)
        {
            Ok(f) => f,
            Err(e) => {
                self.status = format!("Error opening output file: {}", e);
                return;
            }
        };

        use std::io::Write;
        let mut to_write = prefix;
        to_write.push_str(&output_text);

        match file.write_all(to_write.as_bytes()) {
            Ok(_) => {
                self.status = format!(
                    "Appended {} file(s) to {}. Skipped {} file(s).",
                    written_count,
                    output_path.display(),
                    skipped_count
                );
            }
            Err(e) => {
                self.status = format!("Error appending to output file: {}", e);
            }
        }
    }
}
