use tauri::{AppHandle, command, Runtime};

use crate::declare;
use crate::models::*;
use crate::Result;
use crate::PrinterpdfExt;

use crate::window;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.printerpdf().ping(payload)
}

#[command]
pub(crate) fn get_printers() -> String {
  if cfg!(windows) {
    return window::get_printers();
  }

  return "Unsupported OS".to_string();
}

#[command]
pub(crate) fn get_printers_by_name(printername: String) -> String {
  if cfg!(windows) {
    return window::get_printers_by_name(printername);
  }

  return "Unsupported OS".to_string();
}

#[command]
pub async fn print_pdf(
  id: String,
  path: String,
  printer_setting: String,
  remove_after_print: bool,
) -> String {

  if cfg!(windows) {
      let options = declare::PrintOptions {
          id,
          path,
          print_setting: printer_setting,
          remove_after_print
      };
    return window::print_pdf(options);
  }

  return "Unsupported OS".to_string();
}
