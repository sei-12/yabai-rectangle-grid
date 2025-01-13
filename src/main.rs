mod windows_map;
mod yabai;
use std::{fs::File, io::{Read, Write}};

use clap::Parser;
use windows_map::WindowStatus;
use yabai::move_current_window_to_grid;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="/tmp/yabai-rectangle-grid", help = "tmp file path")]
    tmp_file_path: String,
    
    #[arg(short, long, default_value = "2x3", help = "row x column")]
    grid: String,
    
    
    // #[arg(short, long, default_value = "0", help = "ウィンドウを動かしてから動かした後のウィンドウの位置をファイルに書き込むまでの時間。コマンドが実行し終わった時点でウィンドウの位置が更新されていない可能性があるため")]
    // write_delay_ms: u32,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Grid {
    pub rows: u32,
    pub columns: u32,
}

impl TryFrom<String> for Grid {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split('x');
        let row = parts.next().ok_or("missing row")?.parse().map_err(|_| "invalid grid")?;
        let column = parts.next().ok_or("missing column")?.parse().map_err(|_| "invalid grid")?;
        Ok(Self { rows: row, columns: column })
    }
}

fn read_tmp_file_or_empty_string(tmp_file_path: &str) -> String {
    let Ok(mut f) = File::open(tmp_file_path) else {
        return String::new();
    };
    
    
    let mut contents = String::new();
    let result = f.read_to_string(&mut contents);
    
    if result.is_err() {
        return String::new();
    };
    
    contents
}

fn write_tmp_file(tmp_file_path: &str, data: &str) {
    let mut f = File::create(tmp_file_path).expect("failed to create tmp file");
    f.write_all(data.as_bytes()).expect("failed to write to tmp file");
}

#[derive(Debug, PartialEq)]
struct WindowPositionGrid {
    pub rows: u32,
    pub columns: u32,
    pub grid_x: u32,
    pub grid_y: u32,
}

fn calc_next_window_position_grid(prev_status: &WindowStatus, grid: &Grid) -> WindowPositionGrid {
    let rows = grid.rows;
    let columns = grid.columns;
    let mut next_grid_x = prev_status.x_grid;
    let mut next_grid_y = prev_status.y_grid;
    
    if prev_status.x_grid < columns - 1 {
        next_grid_x += 1;
    } 
    else if prev_status.y_grid < rows - 1 {
        next_grid_x = 0;
        next_grid_y += 1;
    } 
    else {
        next_grid_x = 0;
        next_grid_y = 0;
    }
    
    WindowPositionGrid { rows, columns, grid_x: next_grid_x, grid_y: next_grid_y }
}


fn main() {
    let args = Args::parse();

    let tmp_file_path = args.tmp_file_path;
    let grid = match Grid::try_from(args.grid) {
        Ok(grid) => grid,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
    
    let tmp_file_data = read_tmp_file_or_empty_string(tmp_file_path.as_str());
    let mut windows_map = windows_map::WindowsMap::from_or_empty(tmp_file_data);
    let current_window_status = yabai::get_current_forced_window_status();
    let prev_status = windows_map.get(current_window_status.id);
    
    let next_window_position_grid = match prev_status {
        Some(prev_status) => {
            if prev_status.x_px != current_window_status.frame.x_px || prev_status.y_px != current_window_status.frame.y_px {
                None
            }
            else if prev_status.rows != grid.rows || prev_status.columns != grid.columns {
                None
            }
            else if prev_status.w != current_window_status.frame.w || prev_status.h != current_window_status.frame.h {
                None
            }
            else {
                Some(calc_next_window_position_grid(prev_status, &grid))
            }
        },
        None => None
    };

    let next_window_position_grid = match next_window_position_grid {
        Some(next_window_position_grid) => next_window_position_grid,
        None => WindowPositionGrid { rows: grid.rows, columns: grid.columns, grid_x: 0, grid_y: 0 }
    };

    move_current_window_to_grid(
        next_window_position_grid.rows,
        next_window_position_grid.columns, 
        next_window_position_grid.grid_x, 
        next_window_position_grid.grid_y
    );

    
    // let duration = std::time::Duration::from_millis(args.write_delay_ms as u64);
    // thread::sleep(duration);

    let current_window_status = yabai::get_current_forced_window_status();

    let next_status = WindowStatus {
        window_id: current_window_status.id,
        x_px: current_window_status.frame.x_px,
        y_px: current_window_status.frame.y_px,
        w: current_window_status.frame.w,
        h: current_window_status.frame.h,
        rows: grid.rows,
        columns: grid.columns,
        x_grid: next_window_position_grid.grid_x,
        y_grid: next_window_position_grid.grid_y,
    };

    windows_map.insert(next_status.window_id, next_status);
    
    let tmp_file_data:String = windows_map.into();
    write_tmp_file(&tmp_file_path, &tmp_file_data);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_try_from() {
        assert_eq!(Grid::try_from("3x2".to_string()), Ok(Grid { rows: 3, columns: 2 }));
        assert_eq!(Grid::try_from("32x12".to_string()), Ok(Grid { rows: 32, columns: 12 }));

        assert!(Grid::try_from("0b11x0b10".to_string()).is_err());
        assert!(Grid::try_from("not a grid".to_string()).is_err());
        assert!(Grid::try_from("3x3a".to_string()).is_err());
        assert!(Grid::try_from("0xa".to_string()).is_err());
    }
    
    #[test]
    fn test_calc_next_window_position_grid(){
        let prev_status = WindowStatus { window_id: 1, x_px: 0.0, y_px: 0.0, w: 100.0, h: 100.0, rows: 3, columns: 2, x_grid: 0, y_grid: 0 };
        let grid = Grid { rows: 3, columns: 2 };
        assert_eq!(calc_next_window_position_grid(&prev_status, &grid), WindowPositionGrid { rows: 3, columns: 2, grid_x: 1, grid_y: 0 });

        let prev_status = WindowStatus { window_id: 1, x_px: 0.0, y_px: 0.0, w: 100.0, h: 100.0, rows: 3, columns: 2, x_grid: 1, y_grid: 0 };
        let grid = Grid { rows: 3, columns: 2 };
        assert_eq!(calc_next_window_position_grid(&prev_status, &grid), WindowPositionGrid { rows: 3, columns: 2, grid_x: 0, grid_y: 1 });
    }
}
