#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use tauri::{
    CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

mod screenshot;

#[tauri::command]
fn capture_screen() -> Result<Vec<u8>, String> {
    screenshot::capture_full_screen()
}

#[tauri::command]
fn hide_window(window: tauri::Window) -> Result<(), String> {
    window.set_fullscreen(false).map_err(|e| e.to_string())?;
    window
        .set_size(tauri::LogicalSize::new(1, 1))
        .map_err(|e| e.to_string())?;
    window
        .set_position(tauri::LogicalPosition::new(-10000, -10000))
        .map_err(|e| e.to_string())?;
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_to_clipboard(image_data: String) -> Result<(), String> {
    use std::io::Write;
    use std::process::Command;

    let base64_str = if image_data.contains(',') {
        image_data.split(',').nth(1).unwrap_or(&image_data)
    } else {
        &image_data
    };

    let png_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_str)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    let temp_path = std::env::temp_dir().join("screenshot_temp.png");
    let mut file =
        std::fs::File::create(&temp_path).map_err(|e| format!("File create error: {}", e))?;
    file.write_all(&png_bytes)
        .map_err(|e| format!("File write error: {}", e))?;
    file.flush()
        .map_err(|e| format!("File flush error: {}", e))?;
    drop(file);

    #[cfg(target_os = "windows")]
    {
        let path_str = temp_path.to_string_lossy().replace('\\', "\\\\");
        let ps_script = format!(
            "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile('{}'))",
            path_str
        );

        let output = Command::new("powershell")
            .args(&["-NoProfile", "-NonInteractive", "-Command", &ps_script])
            .output()
            .map_err(|e| format!("PowerShell execution error: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("PowerShell failed: {}", stderr));
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("osascript")
            .args(&[
                "-e",
                &format!(
                    "set the clipboard to (read (POSIX file \"{}\") as «class PNGf»)",
                    temp_path.display()
                ),
            ])
            .output()
            .map_err(|e| format!("osascript error: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("osascript failed: {}", stderr));
        }
    }

    let _ = std::fs::remove_file(temp_path);

    Ok(())
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let screenshot = CustomMenuItem::new("screenshot".to_string(), "截图");
    let tray_menu = SystemTrayMenu::new()
        .add_item(screenshot)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "screenshot" => {
                    let window = app.get_window("main").unwrap();
                    let app_handle = app.app_handle();

                    match screenshot::capture_full_screen() {
                        Ok(png_data) => {
                            let base64_data =
                                base64::engine::general_purpose::STANDARD.encode(&png_data);
                            if let Some(monitor) = window.primary_monitor().unwrap() {
                                let size = monitor.size();
                                let scale = monitor.scale_factor();
                                let physical_width = size.width as f64;
                                let physical_height = size.height as f64;
                                let logical_width = physical_width / scale;
                                let logical_height = physical_height / scale;
                                window
                                    .set_size(tauri::LogicalSize::new(
                                        logical_width,
                                        logical_height,
                                    ))
                                    .unwrap();
                                window
                                    .set_position(tauri::LogicalPosition::new(0.0, 0.0))
                                    .unwrap();
                            }
                            window.show().unwrap();
                            window.set_focus().unwrap();
                            app_handle.emit_all("show-screenshot", base64_data).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Screenshot failed: {}", e);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let app_handle = app.handle();
            let window = app.get_window("main").unwrap();
            let window_clone = window.clone();
            let handle_clone = app_handle.clone();

            let mut shortcut = app.global_shortcut_manager();
            shortcut
                .register(
                    "CommandOrControl+Shift+A",
                    move || match screenshot::capture_full_screen() {
                        Ok(png_data) => {
                            let base64_data =
                                base64::engine::general_purpose::STANDARD.encode(&png_data);
                            if let Some(monitor) = window_clone.primary_monitor().unwrap() {
                                let size = monitor.size();
                                let scale = monitor.scale_factor();
                                let physical_width = size.width as f64;
                                let physical_height = size.height as f64;
                                let logical_width = physical_width / scale;
                                let logical_height = physical_height / scale;
                                window_clone
                                    .set_size(tauri::LogicalSize::new(
                                        logical_width,
                                        logical_height,
                                    ))
                                    .unwrap();
                                window_clone
                                    .set_position(tauri::LogicalPosition::new(0.0, 0.0))
                                    .unwrap();
                            }
                            window_clone.show().unwrap();
                            window_clone.set_focus().unwrap();
                            handle_clone
                                .emit_all("show-screenshot", base64_data)
                                .unwrap();
                        }
                        Err(e) => {
                            eprintln!("Screenshot failed: {}", e);
                        }
                    },
                )
                .expect("failed to register hotkey");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            capture_screen,
            save_to_clipboard,
            hide_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
