use std::io::Cursor;
use std::path::Path;

use docx_parser::MarkdownDocument;
use docx_rust::DocxFile;
use pdf_inspector::process_pdf_mem;
use wasm_bindgen::prelude::*;

// Extracts text/markdown from a PDF or DOCX file.
//
// `file_name` is only used to determine the file type from its extension
// (the actual content must be passed in `data`, since wasm running in the
// browser has no access to the real filesystem).
//
// - PDF files are converted to Markdown using `pdf-inspector`.
// - DOCX files are converted to plain text (paragraphs joined by CRLF)
//   using `docx-rust` directly, read from an in-memory buffer.

#[wasm_bindgen]
pub fn doc_to_markdown(file_name: &str, data: &[u8]) -> Result<String, JsValue> {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "pdf" => {
            let result = process_pdf_mem(data)
                .map_err(|err| JsValue::from_str(&format!("Failed to process PDF: {err}")))?;

            Ok(result.markdown.unwrap_or_default())
        }
        "docx" => {
            let docx_file = DocxFile::from_reader(Cursor::new(data)).map_err(|err| {
                JsValue::from_str(&format!("Failed to read DOCX file: {err:?}"))
            })?;

            let docx = docx_file
                .parse()
                .map_err(|err| JsValue::from_str(&format!("Failed to parse DOCX file: {err:?}")))?;

            Ok(docx.document.body.text())
            // let markdown_doc = MarkdownDocument::from_file(file_name);
            // let markdown = markdown_doc.to_markdown(true);
            // Ok(markdown)
        }
        other => Err(JsValue::from_str(&format!(
            "Unsupported file extension: \"{other}\" (expected \"pdf\" or \"docx\")"
        ))),
    }
}
