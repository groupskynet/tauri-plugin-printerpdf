use tauri::{AppHandle, command, Runtime};

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
pub(crate) fn get_printers<R:Runtime>(
    app: AppHandle<R>,
) -> String {
  if cfg!(windows) {
    return window::get_printers();
  }

  return "Unsupported OS".to_string();
}
