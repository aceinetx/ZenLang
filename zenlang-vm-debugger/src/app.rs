use eframe::egui::{self, RichText};
use egui_file_dialog::FileDialog;
use std::{fs, io::Read, path::PathBuf};
use zenlang::{compiler::Compiler, parser::Parser, tokenizer::Tokenizer, vm::VM};
use zenlang_platform_std::*;

pub struct App {
    file_dialog: FileDialog,
    vm: VM,
}

impl App {
    fn load_file(&mut self, path: &PathBuf) {
        let mut file = match fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("error opening file: {}", e);
                return;
            }
        };

        let mut code = String::new();
        if let Err(e) = file.read_to_string(&mut code) {
            eprintln!("error reading file: {}", e);
            return;
        }

        let mut tokenizer = Tokenizer::new(code);
        let mut parser = Parser::new(&mut tokenizer);
        let mut compiler = Compiler::new(&mut parser);

        if let Err(e) = compiler.compile() {
            eprintln!("compile error: {}", e);
            return;
        }

        let module = core::mem::take(compiler.get_module());

        self.vm.platform = Some(Box::new(Platform::new()));

        if let Err(e) = self.vm.load_module(&module) {
            eprintln!("module load error: {}", e);
            return;
        }

        if let Err(e) = self.vm.set_entry_function("main") {
            eprintln!("set entry function error: {}", e);
            return;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            vm: VM::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("ZenLang VM Debugger");
            if ui.button("Run file").clicked() {
                self.file_dialog.pick_file();
            }

            self.file_dialog.update(ui);

            if let Some(path) = self.file_dialog.take_picked() {
                self.load_file(&path);
            }

            ui.horizontal(|ui| {
                if ui.button("step").clicked() && self.vm.error.is_empty() {
                    self.vm.step();
                }

                if self.vm.halted {
                    ui.label("halted");
                }

                ui.label(format!("error: {}", self.vm.error));
            });

            if self.vm.pc.module < self.vm.modules.len() {
                let module = &self.vm.modules[self.vm.pc.module];

                for (i, opcode) in module.opcodes.iter().enumerate() {
                    if self.vm.pc.inst == i {
                        ui.label(
                            RichText::new(format!("> {:?}", opcode))
                                .text_style(egui::TextStyle::Monospace),
                        );
                    } else {
                        ui.label(
                            RichText::new(format!("  {:?}", opcode))
                                .text_style(egui::TextStyle::Monospace),
                        );
                    }
                }
            }
        });
    }
}
