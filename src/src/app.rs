use eframe::egui;
use crate::menu;

pub struct CatEditorApp {
    pub text: String,
}

impl Default for CatEditorApp {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}

impl eframe::App for CatEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //show the menu bar
        menu::show_menu_bar(ctx, self);

        // main text editor area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    let line_count = self.text.lines().count().max(1);
                    let line_number_width = 40.0;

                    ui.allocate_ui_with_layout (
                        egui::vec2(line_number_width, ui.available_height()),
                        egui::Layout::top_down(egui::Align::RIGHT),
                        |ui| {
                            ui.style_mut().spacing.item_spacing.y = 0.0;
                            for line_num in 1..=line_count {
                                ui.label(
                                    egui::RichText::new(format!("{}", line_num))
                                        .color(egui::Color32::from_gray(120))
                                        .monospace()
                                );
                            }
                        },
                    );
                    
                    // the actual text editor section
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.text)
                            .font(egui::TextStyle::Monospace)
                            .frame(false)
                    );
                });
            });
        });
    }
}