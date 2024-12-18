use std::env;

use tauri::{AppHandle, command, Runtime};

use crate::declare;
use crate::fsys;
use crate::models::*;
use crate::Result;
use crate::PrinterpdfExt;

use crate::windows;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.printerpdf().ping(payload)
}


#[command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|create_temp_file')`.
pub(crate) fn create_temp_file(buffer_data: String, filename: String) -> String {
    let dir = env::temp_dir();
    let result = fsys::create_file_from_base64(buffer_data.as_str(), format!("{}{}", dir.display(),filename).as_str());
    if result.is_ok() {
      return format!("{}{}", dir.display(),filename);
    }
  return "".to_owned()
}

#[tauri::command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|remove_temp_file')`.
pub(crate) fn remove_temp_file(filename: String) -> bool {
    let dir = env::temp_dir();
    let result = fsys::remove_file(format!("{}{}", dir.display(),filename).as_str());
    if result.is_ok() {
      return true;
    }
  return false
}

#[command(rename_all = "snake_case")]
pub(crate) fn get_printers() -> String {
  if cfg!(windows) {
    return windows::get_printers();
  }

  return "Unsupported OS".to_string();
}

#[command(rename_all = "snake_case")]
pub(crate) fn get_printers_by_name(printername: String) -> String {
  if cfg!(windows) {
    return windows::get_printers_by_name(printername);
  }

  return "Unsupported OS".to_string();
}

#[command(rename_all = "snake_case")]
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
    return windows::print_pdf(options);
  }

  return "Unsupported OS".to_string();
}


#[command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|get_jobs')`.
pub(crate) fn get_jobs(printername: String) -> String {
  if cfg!(windows) {
    return windows::get_jobs(printername);
  }
  return "Unsupported OS".to_string();
}

#[command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|get_jobs_by_id')`.
pub(crate) fn get_jobs_by_id(printername: String, jobid: String) -> String {
  if cfg!(windows) {
    return windows::get_jobs_by_id(printername, jobid);
  }
  return "Unsupported OS".to_string();
}

#[command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|restart_job')`.
pub(crate) fn resume_job(printername: String, jobid: String) -> String {
  if cfg!(windows) {
    return windows::resume_job(printername,jobid);
  }
  return "Unsupported OS".to_string();
}

#[command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|restart_job')`.
pub(crate) fn restart_job(printername: String, jobid: String) -> String {
  if cfg!(windows) {
    return windows::restart_job(printername,jobid);
  }
  return "Unsupported OS".to_string();
}

#[command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|pause_job')`.
pub(crate) fn pause_job(printername: String, jobid: String) -> String {
  if cfg!(windows) {
    return windows::pause_job(printername, jobid);
  }
  return "Unsupported OS".to_string();
}

#[command(rename_all = "snake_case")]
// this will be accessible with `invoke('plugin:printer|remove_job')`.
pub(crate) fn remove_job(printername: String, jobid: String) -> String {
  if cfg!(windows) {
    return windows::remove_job(printername, jobid);
  }
  return "Unsupported OS".to_string();
}
