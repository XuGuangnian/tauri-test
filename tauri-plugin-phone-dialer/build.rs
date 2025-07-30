const COMMANDS: &[&str] = &[
    "ping",
    "dial_phone_number",
    "request_phone_permission",
    "check_phone_permission",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
