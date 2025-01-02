use std::collections::HashMap;

use arboard::{ClipboardData, ImageData};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ClipboardFormat {
    Text,
    Html,
    Rtf,
    ImageRgba,
    ImagePng,
    ImageSvg,
    Special(String),
    Unsupported,
}

pub struct Clipboard {
    pub format: ClipboardFormat,
    data: ClipboardData,
}

impl Clipboard {
    pub fn new(data: ClipboardData) -> Self {
        let format: ClipboardFormat;

        match &data {
            ClipboardData::Html(_) => format = ClipboardFormat::Html,
            ClipboardData::Image(image_data) => match image_data {
                ImageData::Png(_) => format = ClipboardFormat::ImagePng,
                ImageData::Rgba(_) => format = ClipboardFormat::ImageRgba,
                ImageData::Svg(_) => format = ClipboardFormat::ImageSvg,
            },
            ClipboardData::Rtf(_) => format = ClipboardFormat::Rtf,
            ClipboardData::Text(_) => format = ClipboardFormat::Text,
            ClipboardData::Special((name, _)) => format = ClipboardFormat::Special(name.clone()),
            _ => format = ClipboardFormat::Unsupported,
        }

        Clipboard { format, data }
    }
}

pub trait ClipboardTransform: Sync + Send + 'static {
    fn get_source_format(&self) -> ClipboardFormat;
    fn get_target_format(&self) -> ClipboardFormat;

    fn transform(&self, clipboard: Clipboard) -> Clipboard;
}

pub struct ClipboardTransformManager {
    data: HashMap<ClipboardFormat, Box<dyn ClipboardTransform>>,
}

impl Default for ClipboardTransformManager {
    fn default() -> Self {
        ClipboardTransformManager {
            data: Default::default(),
        }
    }
}

impl ClipboardTransformManager {
    fn add_transform(&mut self, transform: impl ClipboardTransform) {
        self.data
            .insert(transform.get_source_format(), Box::new(transform));
    }

    fn transform(&self, clipboard: Clipboard) -> Clipboard {
        if let Some(transformer) = self.data.get(&clipboard.format) {
            transformer.transform(clipboard)
        } else {
            clipboard
        }
    }

    pub fn transform_data(&self, data: Vec<ClipboardData>) -> Vec<ClipboardData> {
        let clipboard_list: Vec<Clipboard> =
            data.into_iter().map(|item| Clipboard::new(item)).collect();

        clipboard_list
            .into_iter()
            .map(|clipboard| self.transform(clipboard).data)
            .collect()
    }
}
