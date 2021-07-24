/*! # Abstract
rmenu is a one to one rewrite of dmenu in rust for use with redwm.
# Basic Data Flow
* Read from stdin into a vector (maybe hashtable)
* for each line create a assosiated context to be drawn within the bar
* the prompt is drawn, as the user types the list is narrowed down to matche
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
*/
use x11rb::COPY_DEPTH_FROM_PARENT;
pub const HEIGHT: u16 = 22;

pub mod prelude {
    pub use crate::Window;
    pub use x11rb::connection::Connection;
    pub use x11rb::protocol::xproto::Window as X11Window;
    pub use x11rb::protocol::xproto::*;
    pub use x11rb::wrapper::ConnectionExt as _;
}
use crate::prelude::*;

/// Contains the window metadata
pub struct Window {
    /// The window id assigned to the window by the x11 server
    id: X11Window,
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
    class: WindowClass,
    /// The windows Graphical Context values
    values: CreateWindowAux,
}

impl Window {
    /** builds a empty winow that can then be configured
    ```
    # use rmenu::prelude::*;
    # fn main() -> Result<(), Box<dyn std::error::Error>> {
    #    let (conn, screen_num) = x11rb::connect(None)?;
    #    let screen = &conn.setup().roots[screen_num];
    #    let win = conn.generate_id()?;
        let midpoint = (screen.height_in_pixels / 2) as i16;

        // sets the bar to the middle of the screen
        let window = Window::builder(win, &screen).set_y(midpoint);
    #    Ok(())
    # }
    ```
    # Default Values
    * `x`: 0
    * `y`: 0
    * `border`: 0
    * `width`: screen width in pixels
    * `height`:  [`HEIGHT`]
    * `class`: [`WindowClass::INPUT_OUTPUT`]
    * `values`: see [`set_values`]
    */
    pub fn builder(id: X11Window, screen: &Screen) -> Self {
        Window {
            id,
            parent: screen.root,
            x: 0,
            y: 0,
            border: 0,
            width: screen.width_in_pixels,
            height: HEIGHT,
            class: WindowClass::INPUT_OUTPUT,
            values: set_values(&screen),
        }
    }

    /// returns the window id
    pub fn get_id(self) -> X11Window {
	self.id
    }
    pub fn set_id(&mut self, id: X11Window) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_parent(&mut self, parent: X11Window) -> &mut Self {
        self.parent = parent;
        self
    }

    pub fn set_x(&mut self, x: i16) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: i16) -> &mut Self {
        self.y = y;
        self
    }

    pub fn set_border(&mut self, boarder: u16) -> &mut Self {
        self.border = boarder;
        self
    }

    pub fn set_width(&mut self, width: u16) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_height(&mut self, height: u16) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_class(&mut self, class: WindowClass) -> &mut Self {
        self.class = class;
        self
    }

    pub fn set_values(&mut self, values: CreateWindowAux) -> &mut Self {
        self.values = values;
        self
    }
}

/// sets the applicatoin default color sceme and settings
/// # Values
/// * background_pixel(screen.black_pixel)
/// * override_redirect(1)
/// * event_mask(Event::Mask::EXPOSURE)
pub fn set_values(screen: &Screen) -> CreateWindowAux {
    CreateWindowAux::default()
        .background_pixel(screen.black_pixel)
        .override_redirect(1)
        .event_mask(EventMask::EXPOSURE)
}

/** Sets the window properties (see [`Window`] for more configuration options)
```
# use rmenu::prelude::*;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
     let (conn, screen_num) = x11rb::connect(None)?;

     let win = conn.generate_id()?;
     let screen = &conn.setup().roots[screen_num];

     let window = Window::builder(win, &screen);

     set_window_properties(&conn, &window, &screen);
     # Ok(())
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
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        window.id,
        window.parent,
        window.x,
        window.y,
        window.width,
        window.height,
        window.border,
        window.class,
        screen.root_visual,
        &window.values,
    )
    .unwrap();

    let tile = "rmenu";
    conn.change_property8(
        PropMode::REPLACE,
        window.id,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        tile.as_bytes(),
    )
    .unwrap();
}
