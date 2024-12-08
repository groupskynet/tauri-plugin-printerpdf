use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;
mod window;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Printerpdf;
#[cfg(mobile)]
use mobile::Printerpdf;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the printerpdf APIs.
pub trait PrinterpdfExt<R: Runtime> {
  fn printerpdf(&self) -> &Printerpdf<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PrinterpdfExt<R> for T {
  fn printerpdf(&self) -> &Printerpdf<R> {
    self.state::<Printerpdf<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  if cfg!(windows) {
    window::init_windows();
  }
  Builder::new("printerpdf")
    .invoke_handler(tauri::generate_handler![commands::ping, commands::get_printers])
    .setup(|app, api| {
        if cfg!(desktop) {
           window::init(app, api);
        }
        Ok(())
    })
    .build()
}
