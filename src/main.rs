use docx_parser::MarkdownDocument;

use pdf_inspector::process_pdf;
use std::path::Path;

pub fn doc_to_markdown(file_name: &str) -> anyhow::Result<String> {
    let input_path = Path::new(file_name);

    let extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "pdf" => {
            let result = process_pdf(input_path)?;

            if let Some(markdown) = &result.markdown {
                Ok(markdown.clone())
            } else {
                Ok(String::new())
            }
        }
        "docx" => {
            let markdown_doc = MarkdownDocument::from_file(input_path);
            let markdown = markdown_doc.to_markdown(true);
            Ok(markdown)
        }
        other => {
            anyhow::bail!("Unsupported file extension: {}", other)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let a_file = Path::new("/some/path/to/somewhere");

    match doc_to_markdown(&a_file.to_string_lossy()) {
        Ok(md) => println!("{}", md),
        Err(e) => eprintln!("Error converting document: {:?}", e),
    }

    Ok(())
}
