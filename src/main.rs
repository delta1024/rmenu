/** # Abstract
rmenu is a one to one rewrite of dmenu in rust for use with redwm.
# Basic Data Flow
* Read from stdin into a vector (maybe hashtable)
* for each line create a assosiated context to be drawn within the bar
* the prompt is drawn, as the user types the list is narrowed down to matches
* when the user sellects an option it's string is returned to stdout

# Input handeling
```
use std::io::{self, BufRead};
     let stdin = io::stdin();
     let lines: Vec<String> = stdin
         .lock()
         .lines()
         .map(|n| n.expect("coulnd not read from stdin"))
         .collect();
     for i in lines {
         println!("{}", i);
     }
```

# Create X window
```

```
*/
extern crate x11rb;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use std::error::Error;
use x11rb::COPY_DEPTH_FROM_PARENT;
// use x11rb::cursor::Handle as CursorHandle;
// use x11rb::rsource_manager::Database;
use x11rb::wrapper::ConnectionExt as _;

fn set_values(screen: &Screen )-> CreateWindowAux {
    CreateWindowAux::default()
        .background_pixel(screen.black_pixel)
        .override_redirect(1)
        .event_mask(EventMask::EXPOSURE)
}
fn set_window_properties() -> Result<(impl Connection, u32), Box<dyn Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    let screen = &conn.setup().roots[screen_num];
    let width = &conn.setup().roots[screen_num].width_in_pixels;
    let parent = screen.root;
    let values = set_values(screen);
    let win = conn.generate_id().unwrap();
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win,
        parent,
        0,
        0,
        *width,
        22,
        0,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &values,
    )
    .unwrap();

    let tile = "Hello World!";
    conn.change_property8(
        PropMode::REPLACE,
        win,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        tile.as_bytes(),
    )
    .unwrap();
    Ok((conn, win))
}
fn main() {
    let (conn, win) = set_window_properties().unwrap();
    conn.map_window(win).unwrap();
    conn.flush().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(3));
}
