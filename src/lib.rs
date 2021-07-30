/*! # Abstract
rmenu is a one to one rewrite of dmenu in rust for use with redwm with out looking at the dmenu source code.
# Basic Data Flow
## Done
* Read from stdin into a vector
* each line is then concatonated onto the bar.
## TODO
* program will grab keyboard input and display button press in prompot window
* fuzzy search is preformed on user input and checked againsed the piped data
* highest match is highlighted
* when the Return Key is pressed the usser selectoin is outputed to stdout

*/
use std::io::{self, BufRead};
use x11rb::COPY_DEPTH_FROM_PARENT;
pub const HEIGHT: u16 = 22;
pub const BASELINE: i16 = 17;
pub const USER_FONT: &str = "lucidasanstypewriter-12";
pub const FONT_SIZE: usize = 12;
pub const _TEST_VEC: [&str; 2] = ["Hello", "World"];
const TITLE: &str = "rmenu";

pub mod prelude;
use crate::prelude::*;
pub mod events;
pub mod objects;
use objects::TextBox;

/// Returns lines from stdin as  `Vec<String>`.
pub fn parse_std_in() -> RmenuResult<Vec<String>> {
    let bug: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    Ok(bug)
}

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

/** Concatonates the given `Vec<TextBox>` on screen
# Variables
* `conn`: the `X11` Connection
* `window`: the `X11` window id
* `labels`: [`TextBox `] in a `Vec` to concat
*/
pub fn concat_text<C>(conn: &C, window: X11Window, labels: &Vec<TextBox>) -> RmenuResult<()>
where
    C: Connection + RequestConnection,
{
    for i in labels {
        &i.draw(conn, window);
    }
    Ok(())
}
