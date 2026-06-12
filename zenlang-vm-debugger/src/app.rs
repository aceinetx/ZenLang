use eframe::{
    egui::{self, Align, Color32, Panel, RichText, ScrollArea},
    wgpu::Color,
};
use egui_extras::{Column, TableBuilder};
use egui_file_dialog::FileDialog;
use std::{fs, io::Read, path::PathBuf};
use zenlang::{
    compiler::Compiler,
    parser::Parser,
    tokenizer::Tokenizer,
    vm::{ProgramCounter, VM},
};
use zenlang_platform_std::*;

pub struct App {
    file_dialog: FileDialog,
    view_module_id: usize,
    vm: VM,
    error_pc: ProgramCounter,
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

        let mut module = core::mem::take(compiler.get_module());
        module.name = "main".into();
        if let Some(name) = path.file_stem() {
            if let Some(str) = name.to_str() {
                module.name = str.to_string();
            }
        }

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

    fn draw_module_selector(&mut self, ui: &mut egui::Ui) {
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

    fn draw_opcodes(&mut self, ui: &mut egui::Ui) {
        if self.view_module_id < self.vm.modules.len() {
            let module = &self.vm.modules[self.view_module_id];

            ScrollArea::vertical().show(ui, |ui| {
                for (addr, opcode) in module.opcodes.iter().enumerate() {
                    ui.columns(2, |columns| {
                        let current =
                            self.vm.pc.inst == addr && self.vm.pc.module == self.view_module_id;
                        let current_error = addr == self.error_pc.inst
                            && self.error_pc.module == self.view_module_id
                            && !self.vm.error.is_empty();

                        let mut color = Color32::from_rgb(255, 255, 255);
                        if current {
                            color = Color32::from_rgb(0, 0, 255);
                        }
                        if current_error {
                            color = Color32::from_rgb(255, 0, 0);
                        }

                        let resp = columns[0].label(
                            RichText::new(format!("{:<8} | {:?}", addr, opcode))
                                .text_style(egui::TextStyle::Monospace)
                                .color(color),
                        );

                        if current {
                            let visible = columns[0].clip_rect();

                            if !visible.contains(resp.rect.min) || !visible.contains(resp.rect.max)
                            {
                                columns[0].scroll_to_rect(resp.rect, Some(Align::Center));
                            }
                        }

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

    fn draw_vm_panel(&mut self, ui: &mut egui::Ui) {
        Panel::right("vm").resizable(true).show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("step").clicked() && self.vm.error.is_empty() {
                    self.vm.step();
                    if !self.vm.error.is_empty() {
                        self.error_pc = self.vm.pc;
                        self.error_pc.inst -= 1;
                    }
                    self.view_module_id = self.vm.pc.module;
                }

                if self.vm.halted {
                    ui.label("halted");
                }

                ui.label(format!("error: {}", self.vm.error));
            });

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

impl Default for App {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            view_module_id: 0,
            vm: VM::new(),
            error_pc: ProgramCounter::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("ZenLang VM Debugger");
            if ui.button("Run file").clicked() {
                self.file_dialog.pick_file();
            }

            self.file_dialog.update(ui);

            if let Some(path) = self.file_dialog.take_picked() {
                self.load_file(&path);
            }

            ui.separator();

            self.draw_vm_panel(ui);
            self.draw_module_selector(ui);
            self.draw_opcodes(ui);
        });
    }
}
