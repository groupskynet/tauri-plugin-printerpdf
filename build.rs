const COMMANDS: &[&str] = &["ping", "get_printers", "get_printers_by_name", "print_pdf"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
