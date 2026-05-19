pub mod account_ledger_export;
pub mod balance_sheet_export;
pub mod general_ledger_export;
pub mod profit_loss_export;
pub mod trial_balance_export;
pub(crate) mod utils;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use std::io::Cursor;

pub struct DownloadFile {
    content: Vec<u8>,
    filename: String,
    content_type: ContentType,
}

impl DownloadFile {
    pub fn new(content: Vec<u8>, filename: String, content_type: ContentType) -> Self {
        DownloadFile {
            content,
            filename,
            content_type,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for DownloadFile {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'o> {
        Response::build()
            .header(self.content_type)
            .raw_header(
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", self.filename),
            )
            .sized_body(self.content.len(), Cursor::new(self.content))
            .status(Status::Ok)
            .ok()
    }
}
