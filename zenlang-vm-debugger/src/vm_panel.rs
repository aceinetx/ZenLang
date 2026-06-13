use crate::app::App;
use eframe::egui::{self, Panel};
use egui_extras::{Column, TableBuilder};
use zenlang::vm::ProgramCounter;

impl App {
    pub(crate) fn draw_vm_panel(&mut self, ui: &mut egui::Ui) {
        Panel::right("vm").resizable(true).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                // Control panel
                if ui.button("step").clicked() && self.vm.error.is_empty() {
                    self.vm.step();
                    if !self.vm.error.is_empty() {
                        self.error_pc = self.vm.pc;
                        self.error_pc.inst -= 1;
                    }
                    self.view_module_id = self.vm.pc.module;
                    self.want_scroll = true;
                }

                if ui.button("step until stopped").clicked() && self.vm.error.is_empty() {
                    while self.vm.step().is_none() {}
                    if !self.vm.error.is_empty() {
                        self.error_pc = self.vm.pc;
                        self.error_pc.inst -= 1;
                    }
                    self.view_module_id = self.vm.pc.module;
                    self.want_scroll = true;
                }

                if ui.button("restart").clicked() {
                    let path = std::mem::take(&mut self.loaded_path);
                    self.load_file(path);
                }
            });

            // State
            ui.horizontal(|ui| {
                if self.vm.halted {
                    ui.label("halted");
                }

                ui.label(format!("error: {}", self.vm.error));
            });

            // Breakpoints
            ui.strong("Breakpoints");

            let mut remove_breakpoint: Option<ProgramCounter> = None;
            for breakpoint in self.vm.breakpoints.iter() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", breakpoint));
                    if ui.button("x").clicked() {
                        remove_breakpoint = Some(*breakpoint);
                    }
                });
            }
            if let Some(breakpoint) = remove_breakpoint {
                self.vm.breakpoints.remove(&breakpoint);
            }

            // Data
            ui.strong(format!("Return register: {}", self.vm.ret));

            ui.strong("Call stack (most recent call first):");
            for (index, pc) in self.vm.call_stack.iter().enumerate().rev() {
                let mut module_name = "<unknown module>";
                let mut function_name = "<unknown function>";
                for (module_index, module) in self.vm.modules.iter().enumerate() {
                    if module_index == pc.module {
                        module_name = module.name.as_str();

                        for func in module.functions.iter() {
                            if pc.inst >= func.addr {
                                function_name = func.name.as_str();
                            }
                        }
                    }
                }
                ui.label(format!(
                    "{}. {}:{} {}",
                    index, module_name, function_name, pc
                ));
            }

            ui.strong("Stack (most recent value first):");
            for (index, value) in self.vm.stack.iter().enumerate().rev() {
                ui.label(format!("{}. {}", index, value));
            }

            ui.strong("Argument stack (most recent value first):");
            if let Some(args) = self.vm.args.last() {
                for (index, value) in args.iter().enumerate().rev() {
                    ui.label(format!("{}. {}", index, value));
                }
            }

            // Local variables
            let table = TableBuilder::new(ui)
                .column(Column::auto())
                .column(Column::remainder());

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Name");
                    });
                    header.col(|ui| {
                        ui.label("Value");
                    });
                })
                .body(|mut body| {
                    if let Some(scope) = self.vm.scopes.last() {
                        for var in scope.vars.iter() {
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.strong(format!("{}", var.0));
                                });
                                row.col(|ui| {
                                    ui.strong(format!("{}", var.1));
                                });
                            });
                        }
                    }
                });
        });
    }
}
