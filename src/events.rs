//! Event Handling
use crate::concat_text;
use crate::objects;
use crate::prelude::*;
// use crate::HEIGHT;
pub fn handle_event_loop<C>(
    conn: &C,
    win_id: u32,
    gc_id: u32,
    input: Vec<objects::TextBox>,
) -> RmenuResult<()>
where
    C: Connection,
{
    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::Expose(event) => {
                if event.count == 0 {
                    concat_text(conn, win_id, &input)?;
                    objects::x_in_a_box(conn, win_id, gc_id)?;
                    conn.flush()?;
                }
            }

            Event::ButtonPress(event) => {
                for i in &input {
                    if (event.event_x >= i.x1) && (event.event_x <= i.x2) {
                        println!("{}", i.text);
                        std::process::exit(0);
                    }
                }
            }

            Event::Error(_) => eprintln!("Got an unexpected error"),
            _ => eprintln!("Got an unknown event"),
        }
    }
}
