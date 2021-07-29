//! X11 Objects
use crate::FONT_SIZE;
use crate::prelude::*;
use crate::HEIGHT;
use x11rb::rust_connection::ReplyOrIdError;
pub fn x_in_a_box<C>(conn: &C, win_id: X11Window, gc_id: u32) -> RmenuResult<()>
where
    C: Connection + RequestConnection,
{
    let segments = [
        Segment {
            x1: 0,
            y1: 0,
            x2: HEIGHT as i16,
            y2: HEIGHT as i16,
        },
        Segment {
            x1: 0,
            y1: HEIGHT as i16,
            x2: HEIGHT as i16,
            y2: 0,
        },
    ];
    let points = [Point { x: 22, y: 0 }, Point { x: 22, y: 22 }];

    conn.poly_segment(win_id, gc_id, &segments)?;
    conn.poly_line(CoordMode::ORIGIN, win_id, gc_id, &points)?;
    conn.flush()?;
    Ok(())
}

/// Draws text and returns the overall width of the box
pub fn draw_text<C>(
    conn: &C,
    screen: &Screen,
    window: X11Window,
    x1: i16,
    y1: i16,
    label: &str,
    font_name: &str,
) -> RmenuResult<u32>
where
    C: Connection + RequestConnection,
{
    let gc = gc_font_get(conn, screen, window, font_name)?;

    conn.image_text8(window, gc, x1, y1, label.as_bytes())?;


    conn.free_gc(gc)?;
    let len = (label.chars().count() * FONT_SIZE) as u32;
    Ok(len)
}

fn gc_font_get<C>(
    conn: &C,
    screen: &Screen,
    window: X11Window,
    font_name: &str,
) -> Result<Gcontext, ReplyOrIdError>
where
    C: Connection + RequestConnection,
{
    let font = conn.generate_id()?;

    conn.open_font(font, font_name.as_bytes())?;

    let gc = conn.generate_id()?;

    let values = CreateGCAux::default()
        .foreground(screen.black_pixel)
        .background(screen.white_pixel)
        .font(font);
    conn.create_gc(gc, window, &values)?;

    conn.close_font(font)?;

    Ok(gc)
}
