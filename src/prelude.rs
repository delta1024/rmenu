//! general use imports
pub use crate::events::handle_event_loop;
pub use crate::Window;
pub use crate::set_window_properties;
pub use std::error::Error;
pub use x11rb::connection::Connection;
pub use x11rb::connection::RequestConnection;
pub use x11rb::protocol::xproto::Window as X11Window;
pub use x11rb::protocol::xproto::*;
pub use x11rb::protocol::Event;
pub use x11rb::wrapper::ConnectionExt as _;
pub type RmenuResult<T> = Result<T, Box<dyn Error>>;
