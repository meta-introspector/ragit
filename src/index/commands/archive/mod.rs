pub mod block_type;
pub mod decompress;
pub mod erase_lines;
pub mod event_loop;
pub mod init_workers;
pub mod request;
pub mod response;
pub mod status;
pub mod render_dashboard;
pub mod extract_archive;
pub mod extract_archive_worker;
pub mod create;

pub mod request_binary_file;
pub mod request_json_file;

pub use block_type::BlockType;
pub use erase_lines::erase_lines;
pub use request_binary_file::request_binary_file;
pub use request_json_file::request_json_file;

use std::sync::mpsc;

pub struct Channel {
    pub tx: mpsc::Sender<request::Request>,
    pub rx: mpsc::Receiver<response::Response>,
}