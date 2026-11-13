//! Windows registry read helpers for Fluent Terminal.

pub const APP_KEY: &str = "Software\\Fluent\\Fluent Terminal";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Fluent Terminal";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Fluent Terminal") },
        RegValue { name: "Version",     value: String::from("v4.3.14") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Fluent Terminal") },
        RegValue { name: "Publisher",       value: String::from("Fluent") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Fluent Terminal\\uninstall.exe\"") },
    ]
}
