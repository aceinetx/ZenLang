use crate::app::App;
use eframe::egui::{self};

impl App {
    pub(crate) fn draw_module_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("modules");
            for (i, module) in self.vm.modules.iter().enumerate() {
                let mut button = ui.button(format!("{}", module.name));
                if i == self.vm.pc.module {
                    button = button.highlight();
                }

                if button.clicked() {
                    self.view_module_id = i;
                }
            }
        });
    }
}
