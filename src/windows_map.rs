use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct WindowStatus {
    pub window_id: u32,
    pub x_px: f32,
    pub y_px: f32,
    pub x_grid: u32,
    pub y_grid: u32,
    pub w: f32,
    pub h: f32,
    
    pub rows: u32,
    pub columns: u32,
}

pub struct WindowsMap {
    windows: HashMap<u32, WindowStatus>,
}

impl WindowsMap {
    pub fn get(&self, window_id: u32) -> Option<&WindowStatus> {
        self.windows.get(&window_id)
    }
    pub fn insert(&mut self, window_id: u32, window_status: WindowStatus) {
        self.windows.insert(window_id, window_status);
    }

    pub fn from_or_empty(value: String) -> Self {
        let window_map = WindowsMap::try_from(value);
        match window_map {
            Ok(window_map) => window_map,
            Err(_) => {
                let windows = HashMap::new();
                Self { windows }
            }
        }
    }
    
    fn try_from(value: String) -> Result<Self, &'static str> {
        let mut windows = HashMap::new();
        for line in value.lines() {
            let mut parts = line.split_whitespace();
            let window_id = parts.next().ok_or("missing id")?.parse().map_err(|_| "invalid id")?;
            let x = parts.next().ok_or("missing x")?.parse().map_err(|_| "invalid x")?;
            let y = parts.next().ok_or("missing y")?.parse().map_err(|_| "invalid y")?;
            let w = parts.next().ok_or("missing width")?.parse().map_err(|_| "invalid width")?;
            let h = parts.next().ok_or("missing height")?.parse().map_err(|_| "invalid height")?;
            let rows = parts.next().ok_or("missing height")?.parse().map_err(|_| "invalid height")?;
            let columns = parts.next().ok_or("missing height")?.parse().map_err(|_| "invalid height")?;
            let x_grid = parts.next().ok_or("missing x_grid")?.parse().map_err(|_| "invalid x_grid")?;
            let y_grid = parts.next().ok_or("missing x_grid")?.parse().map_err(|_| "invalid x_grid")?;
            windows.insert(window_id, WindowStatus { window_id, x_px: x, y_px: y, w, h , rows, columns,x_grid, y_grid });
        }
        Ok(Self { windows })
    }
}

impl Into<String> for WindowsMap {
    fn into(self) -> String {
        let mut result = String::new();
        for (window_id, window_position) in self.windows {
            result.push_str(&format!("{} {} {} {} {} {} {} {} {}\n", 
                window_id, 
                window_position.x_px,
                window_position.y_px, 
                window_position.w, 
                window_position.h, 
                window_position.rows, 
                window_position.columns,
                window_position.x_grid,
                window_position.y_grid
            ));
        }
        result
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_map_try_from() {
        let value = "1 0 0 100 100 1 1 3 3\n2 100 0 100 100 1 1 3 3\n".to_string();
        let windows_map = WindowsMap::try_from(value).unwrap();
        assert_eq!(windows_map.windows.len(), 2);
        assert_eq!(windows_map.windows[&1].x_px, 0.0);
        assert_eq!(windows_map.windows[&1].y_px, 0.0);
        assert_eq!(windows_map.windows[&1].w, 100.0);
        assert_eq!(windows_map.windows[&1].h, 100.0);
        assert_eq!(windows_map.windows[&2].x_px, 100.0);
        assert_eq!(windows_map.windows[&2].y_px, 0.0);
        assert_eq!(windows_map.windows[&2].w, 100.0);
        assert_eq!(windows_map.windows[&2].h, 100.0);

        let windows_map = WindowsMap::from_or_empty(windows_map.into());
        assert_eq!(windows_map.windows.len(), 2);
        assert_eq!(windows_map.windows[&1].x_px, 0.0);
        assert_eq!(windows_map.windows[&1].y_px, 0.0);
        assert_eq!(windows_map.windows[&1].w, 100.0);
        assert_eq!(windows_map.windows[&1].h, 100.0);
        assert_eq!(windows_map.windows[&2].x_px, 100.0);
        assert_eq!(windows_map.windows[&2].y_px, 0.0);
        assert_eq!(windows_map.windows[&2].w, 100.0);
        assert_eq!(windows_map.windows[&2].h, 100.0);

        let windows_map = WindowsMap::from_or_empty(windows_map.into());
        assert_eq!(windows_map.windows.len(), 2);
        assert_eq!(windows_map.windows[&1].x_px, 0.0);
        assert_eq!(windows_map.windows[&1].y_px, 0.0);
        assert_eq!(windows_map.windows[&1].w, 100.0);
        assert_eq!(windows_map.windows[&1].h, 100.0);
        assert_eq!(windows_map.windows[&2].x_px, 100.0);
        assert_eq!(windows_map.windows[&2].y_px, 0.0);
        assert_eq!(windows_map.windows[&2].w, 100.0);
        assert_eq!(windows_map.windows[&2].h, 100.0);
    }
}