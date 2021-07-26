//! Event Handling
use crate::objects;
use crate::prelude::*;
pub fn handle_event_loop<C>(conn: &C, win_id: u32, gc_id: u32) -> RmenuResult
where
    C: Connection,
{
    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::Expose(event) => {
                if event.count == 0 {
                    objects::x_in_a_box(conn, win_id, gc_id)?;
                }
            }
            Event::Error(_) => eprintln!("Got an unexpected error"),
            _ => eprintln!("Got an unknown event"),
        }
    }
}
