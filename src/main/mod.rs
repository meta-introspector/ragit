use crate::prelude::*;

mod main_run;
mod main_find_root;
mod main_handle_error;

pub use main_run::run;
pub use main_find_root::find_root;
pub use main_handle_error::handle_error;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if let Err(e) = run(args.clone()).await {
        handle_error(e);
        std::process::exit(1);
    }
}