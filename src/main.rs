#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, ViewportBuilder};
use winreg::{enums::*, RegKey, RegValue};

fn main() {
    let viewport = ViewportBuilder {
        inner_size: Some(egui::vec2(410.0, 250.0)),
        resizable: Some(false),
        maximize_button: Some(false),
        ..Default::default()
    };

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "VRC Mouse Sensitivity Changer",
        options,
        Box::new(|cc| Ok(Box::new(VRCMouseSensitivityChanger::new(cc)))),
    );
}

fn percent_to_value(percentage: f64) -> f64 {
    percentage / 100.0
}

pub fn set_mouse_sensitivity(bytes: [u8; 4]) -> Result<(), Box<dyn std::error::Error>> {
    let value: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, bytes[0], bytes[1], bytes[2], bytes[3],
    ];

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let vrchat = hkcu.open_subkey_with_flags("Software\\VRChat\\VRChat", KEY_WRITE)?;

    let reg_value = RegValue {
        bytes: value,
        vtype: REG_BINARY,
    };

    vrchat.set_raw_value("VRC_MOUSE_SENSITIVITY_h864189870", &reg_value)?;

    Ok(())
}

pub fn get_mouse_sensitivity() -> Result<[u8; 4], Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let vrchat = hkcu.open_subkey("Software\\VRChat\\VRChat")?;

    let reg_value = vrchat.get_raw_value("VRC_MOUSE_SENSITIVITY_h864189870")?;

    let bytes = reg_value.bytes;
    let result = [bytes[4], bytes[5], bytes[6], bytes[7]];

    Ok(result)
}

#[derive(Default)]
struct VRCMouseSensitivityChanger {
    sensitivity: f64,
    result_percentage: f64,
    result_value: f64,
    result_bytes: [u8; 4],
}

impl VRCMouseSensitivityChanger {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);

        let bytes = get_mouse_sensitivity().unwrap();
        let value = f64::from_le_bytes([0, 0, 0, 0, bytes[0], bytes[1], bytes[2], bytes[3]]);
        let percentage = value * 100.0;

        Self {
            sensitivity: percentage,
            ..Default::default()
        }
    }
}

impl eframe::App for VRCMouseSensitivityChanger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VRC Mouse Sensitivity Changer");

            ui.horizontal(|ui| {
                ui.label("Sensitivity:");
                ui.add(egui::Slider::new(&mut self.sensitivity, 10.0..=210.0));
            });

            if ui.button("Apply").clicked() {
                let value = percent_to_value(self.sensitivity);
                let bits = value.to_bits().to_le_bytes();

                self.result_percentage = self.sensitivity;
                self.result_value = value;
                self.result_bytes = [bits[4], bits[5], bits[6], bits[7]];

                set_mouse_sensitivity(self.result_bytes).unwrap();
            }

            ui.separator();

            ui.label("Result:");
            ui.horizontal(|ui| {
                ui.label("Percentage:");
                ui.label(format!("{:.2}%", self.result_percentage));
            });
            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.label(format!("{:?}", self.result_value));
            });
            ui.horizontal(|ui| {
                ui.label("Bytes:");
                ui.label(format!(
                    "{:02x},{:02x},{:02x},{:02x}",
                    self.result_bytes[0],
                    self.result_bytes[1],
                    self.result_bytes[2],
                    self.result_bytes[3]
                ));
            });
        });
    }
}
