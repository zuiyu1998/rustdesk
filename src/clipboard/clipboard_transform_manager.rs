use std::collections::HashMap;

use arboard::ClipboardData;

use super::clipboard_transform::ClipboardTransform;

pub struct ClipboardTransformManager {
    data: HashMap<String, Box<dyn ClipboardTransform>>,
}

impl ClipboardTransformManager {
    pub fn transform_clipboard_data(&self, data: ClipboardData) -> Option<ClipboardData> {
        None
    }
}
