use rmenu::prelude::*;
use rmenu::parse_std_in;
fn main() {
    let input = parse_std_in().unwrap();

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
    handle_event_loop(&conn, window.id, gc_id, &screen, input).unwrap();
}
