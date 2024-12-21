use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct YabaiWindowFrame {
    #[serde(rename = "x")]
    pub x_px: f32,
    #[serde(rename = "y")]
    pub y_px: f32,
    pub w: f32,
    pub h: f32
}

#[derive(Serialize, Deserialize)]
pub struct YabaiWindow {
    pub id: u32,
    pub frame: YabaiWindowFrame,
    
    #[serde(rename = "has-focus")]
    has_focus: bool,
}

pub fn get_current_forced_window_status() -> YabaiWindow {
    let output = std::process::Command::new("yabai")
        .arg("-m")
        .arg("query")
        .arg("--windows")
        .output()
        .expect("failed to execute yabai");

    let output = String::from_utf8_lossy(&output.stdout);
    parse_yabai_output_and_find_focused(&output)
}

fn parse_yabai_output_and_find_focused(output: &str) -> YabaiWindow {
    let windows: Vec<YabaiWindow> = serde_json::from_str(&output).expect("failed to parse yabai output");
    let focused_window = windows.into_iter().find(|w| w.has_focus).expect("no focused window");
    focused_window.into()
}

pub fn move_current_window_to_grid(rows: u32, columns: u32, grid_x: u32, grid_y: u32) {
    // yabai -m window --grid 8:8:2:1:4:4
    println!("yabai -m window --grid {}:{}:{}:{}:1:1", rows, columns, grid_x, grid_y);
    std::process::Command::new("yabai")
        .arg("-m")
        .arg("window")
        .arg("--grid")
        .arg(format!("{}:{}:{}:{}:1:1",rows, columns, grid_x, grid_y))
        .output()
        .expect("failed to execute yabai");
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_parse_yabai_output_and_find_focused() {
        let output = r#" [{ "id":19457, "pid":36188, "app":"WezTerm", "title":"yabai", "scratchpad":"", "frame":{ "x":20.0000, "y":60.0000, "w":833.0000, "h":1360.0000 }, "role":"AXWindow", "subrole":"AXStandardWindow", "root-window":true, "display":1, "space":1, "level":0, "sub-level":0, "layer":"normal", "sub-layer":"normal", "opacity":1.0000, "split-type":"none", "split-child":"none", "stack-index":0, "can-move":true, "can-resize":true, "has-focus":true, "has-shadow":true, "has-parent-zoom":false, "has-fullscreen-zoom":false, "has-ax-reference":true, "is-native-fullscreen":false, "is-visible":true, "is-minimized":false, "is-hidden":false, "is-floating":false, "is-sticky":false, "is-grabbed":false },{ "id":16013, "pid":92349, "app":"Code", "title":"yabai.rs — yabai-rectangle-grid", "scratchpad":"", "frame":{ "x":873.0000, "y":60.0000, "w":1667.0000, "h":1360.0000 }, "role":"AXWindow", "subrole":"AXStandardWindow", "root-window":true, "display":1, "space":1, "level":0, "sub-level":0, "layer":"normal", "sub-layer":"normal", "opacity":1.0000, "split-type":"none", "split-child":"none", "stack-index":0, "can-move":true, "can-resize":true, "has-focus":false, "has-shadow":true, "has-parent-zoom":false, "has-fullscreen-zoom":false, "has-ax-reference":true, "is-native-fullscreen":false, "is-visible":true, "is-minimized":false, "is-hidden":false, "is-floating":false, "is-sticky":false, "is-grabbed":false },{ "id":17220, "pid":1433, "app":"Google Chrome", "scratchpad":"", "frame":{ "x":20.0000, "y":60.0000, "w":833.0000, "h":1360.0000 }, "role":"AXWindow", "subrole":"AXStandardWindow", "root-window":true, "display":1, "space":1, "level":0, "sub-level":0, "layer":"normal", "sub-layer":"normal", "opacity":1.0000, "split-type":"none", "split-child":"none", "stack-index":0, "can-move":true, "can-resize":true, "has-focus":false, "has-shadow":true, "has-parent-zoom":false, "has-fullscreen-zoom":false, "has-ax-reference":true, "is-native-fullscreen":false, "is-visible":true, "is-minimized":false, "is-hidden":false, "is-floating":false, "is-sticky":false, "is-grabbed":false },{ "id":13934, "pid":36188, "app":"WezTerm", "title":"..ectangle-grid", "scratchpad":"", "frame":{ "x":660.0000, "y":245.0000, "w":1240.0000, "h":660.0000 }, "role":"AXWindow", "subrole":"AXStandardWindow", "root-window":true, "display":1, "space":1, "level":0, "sub-level":0, "layer":"normal", "sub-layer":"normal", "opacity":1.0000, "split-type":"none", "split-child":"none", "stack-index":0, "can-move":true, "can-resize":true, "has-focus":false, "has-shadow":true, "has-parent-zoom":false, "has-fullscreen-zoom":false, "has-ax-reference":true, "is-native-fullscreen":false, "is-visible":true, "is-minimized":false, "is-hidden":false, "is-floating":false, "is-sticky":false, "is-grabbed":false },{ "id":15296, "pid":1433, "app":"Google Chrome", "title":"ChatGPT - Google Chrome", "scratchpad":"", "frame":{ "x":660.0000, "y":60.0000, "w":1240.0000, "h":1360.0000 }, "role":"AXWindow", "subrole":"AXStandardWindow", "root-window":true, "display":1, "space":2, "level":0, "sub-level":0, "layer":"normal", "sub-layer":"normal", "opacity":1.0000, "split-type":"none", "split-child":"none", "stack-index":0, "can-move":true, "can-resize":true, "has-focus":false, "has-shadow":true, "has-parent-zoom":false, "has-fullscreen-zoom":false, "has-ax-reference":true, "is-native-fullscreen":false, "is-visible":false, "is-minimized":false, "is-hidden":false, "is-floating":false, "is-sticky":false, "is-grabbed":false },{ "id":6068, "pid":64937, "app":"Music", "title":"", "scratchpad":"", "frame":{ "x":1290.0000, "y":60.0000, "w":1250.0000, "h":1360.0000 }, "role":"", "subrole":"", "root-window":true, "display":1, "space":2, "level":0, "sub-level":0, "layer":"normal", "sub-layer":"normal", "opacity":1.0000, "split-type":"none", "split-child":"none", "stack-index":0, "can-move":false, "can-resize":false, "has-focus":false, "has-shadow":true, "has-parent-zoom":false, "has-fullscreen-zoom":false, "has-ax-reference":false, "is-native-fullscreen":false, "is-visible":false, "is-minimized":false, "is-hidden":false, "is-floating":false, "is-sticky":false, "is-grabbed":false }] "#;
        let result = parse_yabai_output_and_find_focused(output);
        assert_eq!(result.id, 19457);
        assert_eq!(result.frame.x_px, 20.0);
        assert_eq!(result.frame.y_px, 60.0);
        assert_eq!(result.frame.w, 833.0);
        assert_eq!(result.frame.h, 1360.0);
    }
}
