use eframe::egui;
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
    pub(crate) file_dialog: FileDialog,
    pub(crate) view_module_id: usize,
    pub(crate) vm: VM,
    pub(crate) error_pc: ProgramCounter,
    pub(crate) loaded_path: PathBuf,
    pub(crate) want_scroll: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            view_module_id: 0,
            vm: VM::new(),
            error_pc: ProgramCounter::new(),
            loaded_path: PathBuf::new(),
            want_scroll: false,
        }
    }
}

impl App {
    pub(crate) fn load_file(&mut self, path: PathBuf) {
        let mut file = match fs::File::open(&path) {
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

        self.vm = VM::new();
        self.vm.platform = Some(Box::new(Platform::new()));

        if let Err(e) = self.vm.load_module(&module) {
            eprintln!("module load error: {}", e);
            return;
        }

        if let Err(e) = self.vm.set_entry_function("main") {
            eprintln!("set entry function error: {}", e);
            return;
        }

        self.loaded_path = path;
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
                self.load_file(path);
            }

            ui.separator();

            self.draw_vm_panel(ui);
            self.draw_module_selector(ui);
            self.draw_opcodes(ui);
        });
    }
}
