import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:phone-dialer|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export interface DialPhoneResult {
  success: boolean;
  message: string;
}

export async function dialPhoneNumber(
  phoneNumber: string
): Promise<DialPhoneResult> {
  return await invoke<DialPhoneResult>(
    "plugin:phone-dialer|dial_phone_number",
    {
      payload: {
        phoneNumber,
      },
    }
  );
}

export interface PermissionResult {
  success: boolean;
  message: string;
}

export async function requestPhonePermission(): Promise<PermissionResult> {
  return await invoke<PermissionResult>(
    "plugin:phone-dialer|request_phone_permission"
  );
}

export async function checkPhonePermission(): Promise<PermissionResult> {
  return await invoke<PermissionResult>(
    "plugin:phone-dialer|check_phone_permission"
  );
}
