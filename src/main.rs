use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)  // Integrate egui
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn ui_system(mut contexts: EguiContexts) {
    egui::Window::new("Media Editor").show(contexts.ctx_mut(), |ui| {
        ui.label("Welcome to your editor!");
        if ui.button("Import Video").clicked() {
            // Handle import
        }
    });
}