use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, BufRead, Write};
use crossterm::{
    cursor, ExecutableCommand, terminal::{self, ClearType}, event::{self, Event, KeyCode}
};

//////const MAP_WIDTH: usize = MAP[0].len();
//const MAP_HEIGHT: usize = MAP.len(); //bug here if unicode
const FOV: f64 = PI / 2.0;  // Field of view (60 degrees)

struct Player {
    x: f64,
    y: f64,
    angle: f64,
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    //map must be uniform height
    // Load map from file
    let map_file = if args.len() > 1 { &args[1] } else { "map.txt" };
    let map = load_map(map_file).expect("Failed to load map");

    let mut stdout = io::stdout();
    let mut player = Player {
        x: 2.0,
        y: 2.0,
        angle: 0.0,//
    };

    // Setup terminal
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    loop {
        render(&mut stdout, &player, &map);
        let _ = stdout.flush();
        if handle_input(&mut player).is_some() {
            break;
        }
    }

    // Restore terminal state
    terminal::disable_raw_mode().unwrap();
    stdout.execute(cursor::Show).unwrap();
}

fn load_map(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let map: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(map)
}

fn render<W: Write>(stdout: &mut W, player: &Player, map: &[String]) {
    // Query terminal size (width, height)
    let (screen_width, screen_height) = terminal::size().unwrap();
    
    // Adjust the size to prevent flicker (ensure even numbers)
    let screen_width = (screen_width as usize).max(10);   // Min width of 10 to avoid too small displays
    let screen_height = (screen_height as usize).max(10); // Min height of 10

    let map_width = map[0].len();
    let map_height = map.len();

    // Move the cursor back to the top-left at the beginning of every render
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    for y in 0..screen_height {
        for x in 0..screen_width {
            let ray_angle = player.angle - FOV / 2.0 + (x as f64 / screen_width as f64) * FOV;
            let distance = cast_ray(player, ray_angle, map);
            
            // Ensure we don't divide by zero and clamp the distance
            let distance = distance.max(0.1); // Prevent dividing by zero
            
            let wall_height = (screen_height as f64 / distance) as isize; // Use signed integer for wall height
            let wall_height = wall_height.clamp(0, screen_height as isize); // Clamp the wall height to the screen height

            // Check if current row (y) is for ceiling, wall, or floor
            if (y as isize) < (screen_height as isize - wall_height) / 2 {
                write!(stdout, " ").unwrap();  // Ceiling
            } else if (y as isize) < (screen_height as isize + wall_height) / 2 {
                write!(stdout, "▓").unwrap();  // Wall
            } else {
                write!(stdout, "░").unwrap();  // Floor
            }
        }
        write!(stdout, "\r\n").unwrap();  // Move to the next line
    }
}

fn cast_ray(player: &Player, ray_angle: f64, map: &[String]) -> f64 {
    let mut distance = 0.0;
    let step_size = 0.1;
    let mut hit_wall = false;
    let mut test_x;
    let mut test_y;

    while !hit_wall && distance < 20.0 {
        distance += step_size;
        test_x = (player.x + ray_angle.cos() * distance) as isize;
        test_y = (player.y + ray_angle.sin() * distance) as isize;

        // Ensure we're not going out of bounds
        if test_x < 0 || test_x as usize >= map[0].len() || test_y < 0 || test_y as usize >= map.len() {
            hit_wall = true;
            distance = 20.0;
        } else if map[test_y as usize].as_bytes()[test_x as usize] == b'#' {
            hit_wall = true;
        }
    }
    distance
}

fn handle_input(player: &mut Player) -> Option<()> {
    use crossterm::event::{KeyEventKind};

    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') => return Some(()),  // Exit the loop
                    KeyCode::Char('a') => player.angle -= 0.1,
                    KeyCode::Char('d') => player.angle += 0.1,
                    KeyCode::Char('w') => {
                        player.x += player.angle.cos() * 0.1;
                        player.y += player.angle.sin() * 0.1;
                    }
                    KeyCode::Char('s') => {
                        player.x -= player.angle.cos() * 0.1;
                        player.y -= player.angle.sin() * 0.1;
                    }
                    _ => {}
                }
            }
        }
    }
    None
}
