use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Window {
    pub id: u32,
    pub pid: u32,
    pub app: String,
    pub title: String,
    pub frame: Frame,
    pub role: String,
    pub subrole: String,
    pub display: u8,
    pub space: u8,
    pub level: u16,
    pub opacity: f32,
    pub split_type: String,
    pub stack_index: u8,
    pub can_move: bool,
    pub can_resize: bool,
    pub has_focus: bool,
    pub has_shadow: bool,
    pub has_border: bool,
    pub has_parent_zoom: bool,
    pub has_fullscreen_zoom: bool,
    pub is_native_fullscreen: bool,
    pub is_visible: bool,
    pub is_minimized: bool,
    pub is_hidden: bool,
    pub is_floating: bool,
    pub is_sticky: bool,
    pub is_topmost: bool,
    pub is_grabbed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Space {
    pub id: u32,
    pub uuid: String,
    pub index: u32,
    pub label: String,
    // Can't use `type` as ident
    #[serde(rename = "type")]
    pub space_type: String,
    pub display: u8,
    pub windows: Vec<u32>,
    pub first_window: u32,
    pub last_window: u32,
    pub has_focus: bool,
    pub is_visible: bool,
    pub is_native_fullscreen: bool,
}
