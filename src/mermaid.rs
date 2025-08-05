use zed_extension_api as zed;

struct MermaidPreviewExtension;

impl zed::Extension for MermaidPreviewExtension {
    fn new() -> Self {
        println!("[mermaid-preview] Extension initialized");
        Self
    }

    fn on_buffer_changed(&mut self, buffer: &zed::Buffer, _changes: &[zed::Change]) {
        if buffer.language_id() != "markdown" {
            return;
        }

        let text = buffer.text();
        let mermaid_code = extract_mermaid_block(text);
        if !mermaid_code.is_empty() {
            write_preview_html(&mermaid_code);
        }
    }
}

fn extract_mermaid_block(text: &str) -> String {
    let start_tag = "```mermaid";
    let end_tag = "```";

    let mut inside_block = false;
    let mut block_lines = vec![];

    for line in text.lines() {
        if line.trim_start().starts_with(start_tag) {
            inside_block = true;
            continue;
        }
        if inside_block && line.trim_start().starts_with(end_tag) {
            break;
        }
        if inside_block {
            block_lines.push(line);
        }
    }

    block_lines.join("\n")
}

fn write_preview_html(code: &str) {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
</head>
<body>
  <div class="mermaid">{}</div>
  <script>mermaid.initialize({{ startOnLoad: true }});</script>
</body>
</html>"#,
        code
    );

    let _ = std::fs::write("/tmp/mermaid_preview.html", html);

    // Open only once or on significant changes if needed
    let _ = std::process::Command::new("xdg-open") // "open" for macOS
        .arg("/tmp/mermaid_preview.html")
        .spawn();
}

zed::register_extension!(MermaidPreviewExtension);
