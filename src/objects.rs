//! X11 Objects
use crate::prelude::*;
use crate::HEIGHT;
pub fn x_in_a_box<C>(conn: &C, win_id: X11Window, gc_id: u32) -> RmenuResult
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
