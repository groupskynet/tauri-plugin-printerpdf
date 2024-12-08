use std::{sync::mpsc, io::Write};
use std::thread;
use std::env;
use std::fs::File;
use std::process::{Command, Output};

use serde::de::DeserializeOwned;
use tauri::plugin::PluginApi;
use tauri::{AppHandle, Runtime};



/**
 * Create sm.exe to temp
 */
fn create_file(path: String, bin: &[u8]) -> std::io::Result<()> {
    let mut f = File::create(format!("{}sm.exe", path))?;
    f.write_all(bin)?;
  
    f.sync_all()?;
    Ok(())
}

  
/**
 * init sm.exe
 */
pub fn init_windows() {
    let sm = include_bytes!("bin/sm");
    println!("sm.exe");
    let dir: std::path::PathBuf = env::temp_dir();
    let result: Result<(), std::io::Error>  = create_file(dir.display().to_string(),sm);
    if result.is_err() {
        panic!("Gagal")
    }
}

pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) {
  init_windows();
}

/**
 * Get printers on windows using powershell
 */
pub fn get_printers() -> String {
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();
    // Spawn a new thread
    thread::spawn(move || {
        let output: Output = Command::new("powershell")
            .args(["Get-Printer | Select-Object Name, DriverName, JobCount, PrintProcessor, PortName, ShareName, ComputerName, PrinterStatus, Shared, Type, Priority | ConvertTo-Json"])
            .output().expect("failed to execute command");

        sender.send(String::from_utf8(output.stdout).unwrap()).unwrap();
    });

    // Do other non-blocking work on the main thread

    // Receive the result from the spawned thread
    let result: String = receiver.recv().unwrap();

    return result
}

