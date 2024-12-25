use bevy::prelude::*;
use bevy_infig::IntegratedConfig;

pub fn start(cfg: IntegratedConfig) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Momori".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
