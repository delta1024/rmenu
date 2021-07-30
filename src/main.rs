use rmenu::objects::TextBox;
use rmenu::parse_std_in;
use rmenu::prelude::*;
use rmenu::{BASELINE, USER_FONT};
fn main() {
    let std_in = parse_std_in().unwrap();
    let mut input = {
        let mut vec = Vec::new();
        for i in &std_in {
            let n = TextBox::new(&i);
            vec.push(n);
        }
        vec
    };

    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let win = conn.generate_id().unwrap();
    let screen = &conn.setup().roots[screen_num];

    let window = Window::builder(win, &screen);
    set_window_properties(&conn, &window, &screen);

    let values = CreateGCAux::default().foreground(screen.black_pixel);
    let gc_id = conn.generate_id().unwrap();
    conn.create_gc(gc_id, window.id, &values).unwrap();

    conn.map_window(window.id).unwrap();
    conn.flush().unwrap();

    {
        let mut x = 24;

        for i in &mut input {
            i.set_gc(&conn, &screen, window.id, USER_FONT)
                .unwrap()
                .set_x(x)
                .set_y(BASELINE);
            x = i.x2;
        }
    }

    handle_event_loop(&conn, window.id, gc_id, input).unwrap();
}
