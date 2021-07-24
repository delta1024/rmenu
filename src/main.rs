use rmenu::prelude::*;
use rmenu::set_window_properties;
fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    let win = conn.generate_id().unwrap();
    let screen = &conn.setup().roots[screen_num];

    let window = Window::builder(win, &screen);

    set_window_properties(&conn, &window, &screen);

    conn.map_window(window.get_id()).unwrap();
    conn.flush().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
