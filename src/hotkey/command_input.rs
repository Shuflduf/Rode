use eframe::egui;

pub struct CommandInput {
    pub open: bool,
    pub input: String,
}

impl Default for CommandInput {
    fn default() -> Self {
        Self {
            open: false,
            input: String::new(),
        }
    }
}

impl CommandInput {
    pub fn open(&mut self) {
        self.open = true;
        self.input.clear();
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<String> {
        if !self.open { return None; }

        let mut submitted_command = None;

        egui::Window::new("cmd_input_modal")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 100.0))
            .fixed_size(egui::vec2(500.0, 40.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(":").text_style(egui::TextStyle::Monospace).strong());
                    
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.input)
                            .hint_text("Enter Vim command...")
                            .font(egui::TextStyle::Monospace) 
                            .lock_focus(true)
                    );

                    response.request_focus();

                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        submitted_command = Some(self.input.clone());
                        self.close();
                    }
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.close();
                    }
                });
            });

        submitted_command
    }
}