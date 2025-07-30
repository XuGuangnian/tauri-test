use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<PhoneDialer<R>> {
  Ok(PhoneDialer(app.clone()))
}

/// Access to the phone-dialer APIs.
pub struct PhoneDialer<R: Runtime>(AppHandle<R>);

impl<R: Runtime> PhoneDialer<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
