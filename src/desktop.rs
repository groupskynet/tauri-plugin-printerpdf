use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;
use crate::window;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Printerpdf<R>> {
  window::init_windows();
  Ok(Printerpdf(app.clone()))
}

/// Access to the printerpdf APIs.
pub struct Printerpdf<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Printerpdf<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
