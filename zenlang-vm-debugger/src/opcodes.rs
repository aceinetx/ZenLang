use crate::app::App;
use eframe::egui::{self, Align, Color32, RichText, ScrollArea};
use zenlang::vm::ProgramCounter;

impl App {
    pub(crate) fn draw_opcodes(&mut self, ui: &mut egui::Ui) {
        if self.view_module_id < self.vm.modules.len() {
            let module = &self.vm.modules[self.view_module_id];

            ScrollArea::vertical().show(ui, |ui| {
                for (addr, opcode) in module.opcodes.iter().enumerate() {
                    let addr_pc = ProgramCounter::with(self.view_module_id, addr);
                    ui.columns(2, |columns| {
                        columns[0].horizontal(|ui| {
                            // Address label
                            let is_breakpoint = self.vm.breakpoints.contains(&addr_pc);
                            let mut address_color = Color32::from_rgb(255, 255, 255);
                            if is_breakpoint {
                                address_color = Color32::from_rgb(255, 0, 0)
                            }
                            let addr_label = ui.label(
                                RichText::new(format!("{:<8}", addr))
                                    .text_style(egui::TextStyle::Monospace)
                                    .color(address_color),
                            );
                            if addr_label.clicked() {
                                if self.vm.breakpoints.contains(&addr_pc) {
                                    self.vm.breakpoints.remove(&addr_pc);
                                } else {
                                    self.vm.breakpoints.insert(addr_pc);
                                }
                            }

                            // Opcode label
                            let current = self.vm.pc == addr_pc;
                            let current_error =
                                addr_pc == self.error_pc && !self.vm.error.is_empty();

                            let mut opcode_color = Color32::from_rgb(255, 255, 255);
                            if current {
                                opcode_color = Color32::from_rgb(0, 0, 255);
                            }
                            if current_error {
                                opcode_color = Color32::from_rgb(255, 0, 0);
                            }

                            let resp = ui.label(
                                RichText::new(format!("{:?}", opcode))
                                    .text_style(egui::TextStyle::Monospace)
                                    .color(opcode_color),
                            );

                            if current && self.want_scroll {
                                let visible = ui.clip_rect();

                                if !visible.contains(resp.rect.min)
                                    || !visible.contains(resp.rect.max)
                                {
                                    ui.scroll_to_rect(resp.rect, Some(Align::Center));
                                }

                                self.want_scroll = false;
                            }
                        });

                        // Function name label
                        for function in module.functions.iter() {
                            if function.addr == addr {
                                columns[1].label(format!("{}", function.name));
                            }
                        }
                    })
                }
            });
        }
    }
}
