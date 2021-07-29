/*! # Abstract
rmenu is a one to one rewrite of dmenu in rust for use with redwm with out looking at the dmenu source code.
# Basic Data Flow
* Read from stdin into a vector (maybe hashtable)
* for each line create a assosiated context to be drawn within the bar
* the prompt is drawn, as the user types the list is narrowed down to matche (use fuzzy sort for this)
* when the user sellects an option it's string is returned to stdout

# Main
```
use rmenu::prelude::*;
fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let win = conn.generate_id().unwrap();
    let screen = &conn.setup().roots[screen_num];

    let gc_id = conn.generate_id().unwrap();

    let window = Window::builder(win, &screen);

    let values = CreateGCAux::default().foreground(screen.black_pixel);
    set_window_properties(&conn, &window, &screen);

    conn.create_gc(gc_id, window.id, &values).unwrap();

    conn.map_window(window.id).unwrap();
    conn.flush().unwrap();
    handle_event_loop(&conn, window.id, gc_id).unwrap();
}
```
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
*/
use x11rb::COPY_DEPTH_FROM_PARENT;
pub const HEIGHT: u16 = 22;
// pub const USER_FONT: &str = "-adobe-times-medium-i-normal--8-80-75-75-p-42-iso10646-1";
pub const USER_FONT: &str = "lucidasanstypewriter-12";
pub const FONT_SIZE: usize = 12;
const TITLE: &str = "rmenu";

pub mod prelude;
use crate::prelude::*;
pub mod events;
pub mod objects;

#[derive(Default)]
/// Contains the window metadata
pub struct Window {
    /// The window id assigned to the window by the X11 server
    pub id: X11Window,
    /// The id of the parent window
    parent: X11Window,
    /// The `x` coordonate of the upper left corner of the window
    x: i16,
    /// The `y` coordonate of the upper left corner of the window
    y: i16,
    /// The border width
    border: u16,
    /// The width of the window
    width: u16,
    /// The height of the window
    height: u16,
    /// The windows class
    class: Option<WindowClass>,
    /// The windows Graphical Context values
    values: CreateWindowAux,
}

impl Window {
    /** builds a empty winow that can then be configured
    
    see [`set_window_properties`] for a full example of how to set up the X connection.
    # Default Values
    * `x`: 0
    * `y`: 0
    * `border`: 0
    * `width`: screen width in pixels
    * `height`:  [`HEIGHT`]
    * `class`: [`None`] 
    * `values`: see [`set_values`]
    */
    pub fn builder(id: X11Window, screen: &Screen) -> Self {
        Window {
            id,
            parent: screen.root,
            width: screen.width_in_pixels,
            height: HEIGHT,
            values: set_values(&screen),
            ..Default::default()
        }
    }

}

/// sets the application default color sceme and settings
/// # Values
/// * `background_pixel(screen.black_pixel)`
/// * `override_redirect(1)`
/// * `event_mask(Event::Mask::EXPOSURE)`
pub fn set_values(screen: &Screen) -> CreateWindowAux {
    CreateWindowAux::default()
        .background_pixel(screen.white_pixel)
        .override_redirect(1)
        .event_mask(EventMask::EXPOSURE)
}

/** Sets the window properties (see [`Window`] for more configuration options)
```
# use rmenu::prelude::*;
# fn main() -> RmenuResult {
    use rmenu::set_window_properties;

    let (conn, screen_num) = x11rb::connect(None)?;

    let win = conn.generate_id()?;
    let screen = &conn.setup().roots[screen_num];

    let window = Window::builder(win, &screen);

    set_window_properties(&conn, &window, &screen);
#   Ok(())
# }
```

# Variables
* `conn`: A reference to the X connection.
* `window`: A reference to the window struct.
* `screen`: A reference to the active screen.
*/
pub fn set_window_properties<C>(conn: &C, window: &Window, screen: &Screen)
where
    C: Connection,
{
    let class = match window.class {
        Some(c) => c,
        None => WindowClass::INPUT_OUTPUT,
    };
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        window.id,
        window.parent,
        window.x,
        window.y,
        window.width,
        window.height,
        window.border,
        class,
        screen.root_visual,
        &window.values,
    )
    .unwrap();

    let tile = TITLE;
    conn.change_property8(
        PropMode::REPLACE,
        window.id,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        tile.as_bytes(),
    )
    .unwrap();
}
