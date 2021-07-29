//! Event Handling
use crate::objects;
use crate::prelude::*;
pub fn handle_event_loop<C>(conn: &C, win_id: u32, gc_id: u32, screen: &Screen) -> RmenuResult<()>
where
    C: Connection,
{
    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::Expose(event) => {
                if event.count == 0 {
		    let len = objects::draw_text(conn, screen, win_id, 25, 17, "bye", crate::USER_FONT)?;
		    objects::draw_text(conn, screen, win_id, (len + 25) as i16, 17, "word", crate::USER_FONT)?;
                    objects::x_in_a_box(conn, win_id, gc_id)?;
		    println!("{}", len);
                }
            }
            Event::Error(_) => eprintln!("Got an unexpected error"),
            _ => eprintln!("Got an unknown event"),
        }
    }
}
