// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 使用特性来控制相机插件的导入
#[cfg(feature = "camera")]
use tauri_plugin_camera;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    builder = builder.plugin(tauri_plugin_opener::init());
    builder = builder.plugin(tauri_plugin_os::init());

    // 只在启用 camera 特性时初始化相机插件

    #[cfg(feature = "camera")]
    {
        builder = builder.plugin(tauri_plugin_camera::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
