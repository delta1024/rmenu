//! X11 Objects
use crate::prelude::*;
use crate::FONT_SIZE;
use crate::HEIGHT;
/// Holds information about text location and context
#[derive(Debug, Clone, Default, Copy)]
pub struct TextBox<'a> {
    text: &'a str,
    x1: i16,
    /// The end point for the text box
    pub x2: i16,
    y: i16,
    context: Gcontext,
}

impl<'a> TextBox<'a> {
    pub fn new(text: &'a str) -> Self {
        TextBox {
            text,
            ..TextBox::default()
        }
    }

    /// Sets the [`Gcontext`] value for the [`TextBox`].
    /// TODO: extract values to it's own function/section.
    pub fn set_gc<C>(
        &mut self,
        conn: &C,
        screen: &Screen,
        window: X11Window,
        font_name: &str,
    ) -> RmenuResult<&mut Self>
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
        self.context = gc;

        Ok(self)
    }

    pub fn set_x(&mut self, x: i16) -> &mut Self {
        self.x1 = x;
        self.x2 = (self.text.len() * FONT_SIZE) as i16 + self.x1;
        self
    }
    pub fn set_y(&mut self, y: i16) -> &mut Self {
        self.y = y;
        self
    }

    pub fn draw<C>(self, conn: &C, window: X11Window) -> RmenuResult<()>
    where
        C: Connection + RequestConnection,
    {
        let gc = self.context;
        let x = self.x1;
        let y = self.y;
        let text = self.text;

        conn.image_text8(window, gc, x, y, text.as_bytes())?;

        conn.free_gc(gc)?;
        Ok(())
    }
}


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
    Ok(())
}
