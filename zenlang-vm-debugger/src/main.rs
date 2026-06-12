use eframe::egui;
use zenlang_vm_debugger::app::App;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ZenLang VM Debugger",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
