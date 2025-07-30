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

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::PhoneDialer;
#[cfg(mobile)]
use mobile::PhoneDialer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the phone-dialer APIs.
pub trait PhoneDialerExt<R: Runtime> {
    fn phone_dialer(&self) -> &PhoneDialer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PhoneDialerExt<R> for T {
    fn phone_dialer(&self) -> &PhoneDialer<R> {
        self.state::<PhoneDialer<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("phone-dialer")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::dial_phone_number,
            commands::request_phone_permission,
            commands::check_phone_permission
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let phone_dialer = mobile::init(app, api)?;
            #[cfg(desktop)]
            let phone_dialer = desktop::init(app, api)?;
            app.manage(phone_dialer);
            Ok(())
        })
        .build()
}
