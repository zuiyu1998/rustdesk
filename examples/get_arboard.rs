use arboard::ClipboardFormat;
use hbb_common::anyhow;
use librustdesk::clipboard::clipboard_transform::Clipboard;

const SUPPORTED_FORMATS: &[ClipboardFormat] = &[
    ClipboardFormat::Text,
    ClipboardFormat::Html,
    ClipboardFormat::Rtf,
    ClipboardFormat::ImageRgba,
    ClipboardFormat::ImagePng,
    ClipboardFormat::ImageSvg,
];

fn main() -> anyhow::Result<()> {
    let mut board = arboard::Clipboard::new()?;

    let data = board.get_formats(SUPPORTED_FORMATS)?;

    for format_data in data.into_iter() {
        let clipboard = Clipboard::new(format_data);

        println!("format: {:?}", clipboard.format);
    }

    Ok(())
}
