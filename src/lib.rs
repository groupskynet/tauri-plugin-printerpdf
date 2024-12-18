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
mod declare;
mod error;
mod fsys;
mod models;
mod windows;

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
    Builder::new("printerpdf")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::create_temp_file,
            commands::remove_temp_file,
            commands::get_printers,
            commands::get_printers_by_name,
            commands::print_pdf,
            commands::get_jobs,
            commands::get_jobs_by_id,
            commands::resume_job,
            commands::restart_job,
            commands::pause_job,
            commands::remove_job,
        ])
        .setup(|app, api| {
            if cfg!(desktop) {
                windows::init(app, api);
            }
            Ok(())
        })
        .build()
}
