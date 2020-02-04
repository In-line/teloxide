#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/dev/logo.svg",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png"
)]
#![allow(clippy::match_bool)]

pub use bot::Bot;
pub use errors::{ApiErrorKind, DownloadError, RequestError};
pub use session;

mod errors;
mod net;

mod bot;
pub mod dispatching;
pub mod prelude;
pub mod requests;
pub mod types;
pub mod utils;
pub mod session;
