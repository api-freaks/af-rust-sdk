//! File-name / MIME detection for multipart uploads.
//!
//! Upload fields are plain bytes, but the APIFreaks API decides the file type
//! from the upload's file-name extension (a bare `file` name gets 415). This
//! sniffs the content signature of every format the upload endpoints accept.
//! (Added by af-sdk-generation scripts/lib/fix_rust_multipart_filename.py.)

fn sniff(bytes: &[u8]) -> (&'static str, &'static str) {
    let starts = |sig: &[u8]| bytes.starts_with(sig);
    if starts(b"%PDF") {
        ("file.pdf", "application/pdf")
    } else if starts(b"\x89PNG\r\n\x1a\n") {
        ("file.png", "image/png")
    } else if starts(b"\xFF\xD8\xFF") {
        ("file.jpg", "image/jpeg")
    } else if starts(b"GIF87a") || starts(b"GIF89a") {
        ("file.gif", "image/gif")
    } else if starts(b"II*\x00") || starts(b"MM\x00*") {
        ("file.tiff", "image/tiff")
    } else if starts(b"BM") && bytes.len() >= 26 && bytes[6..10] == [0, 0, 0, 0] {
        ("file.bmp", "image/bmp")
    } else if bytes.len() >= 128 && bytes[0] == 0x0A && bytes[1] <= 5 && bytes[2] == 1 {
        ("file.pcx", "image/x-pcx")
    } else if starts(b"PK\x03\x04") || starts(b"PK\x05\x06") {
        ("file.zip", "application/zip")
    } else if bytes.len() >= 12 && starts(b"RIFF") && &bytes[8..12] == b"WEBP" {
        ("file.webp", "image/webp")
    } else if !bytes.is_empty() && !bytes.contains(&0) && std::str::from_utf8(bytes).is_ok() {
        // The only text upload the API takes is the bulk-template CSV.
        ("file.csv", "text/csv")
    } else {
        ("file", "application/octet-stream")
    }
}

/// File name (with the right extension) to send for an uploaded byte buffer.
pub fn upload_file_name(bytes: &[u8]) -> &'static str {
    sniff(bytes).0
}

/// MIME type to send for an uploaded byte buffer.
pub fn upload_mime_type(bytes: &[u8]) -> &'static str {
    sniff(bytes).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sniff_known_types() {
        assert_eq!(upload_file_name(b"%PDF-1.7\n..."), "file.pdf");
        assert_eq!(upload_mime_type(b"\x89PNG\r\n\x1a\n...."), "image/png");
        assert_eq!(upload_file_name(b"\xFF\xD8\xFF\xE0..."), "file.jpg");
        assert_eq!(upload_file_name(b"name,email\na,b\n"), "file.csv");
        assert_eq!(upload_file_name(b"PK\x03\x04...."), "file.zip");
        assert_eq!(upload_file_name(&[0u8, 1, 2, 3]), "file");
    }
}
