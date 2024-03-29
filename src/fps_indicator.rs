use bevy::prelude::*;
use bevy::diagnostic::{ Diagnostics, FrameTimeDiagnosticsPlugin };

// I copied this from flock-rs (https://github.com/JohnPeel/flock-rs)

#[derive(Clone, Debug, Resource)]
pub struct OnScreenFpsConfig {
    pub font: &'static str,
    pub text_style: TextStyle,
    pub style: Style
}

impl Default for OnScreenFpsConfig {
    fn default() -> Self {
        OnScreenFpsConfig {
            font: "fonts/Inconsolata.ttf",
            text_style: Default::default(),
            style: Default::default()
        }
    }
}

#[derive(Clone, Debug, Component)]
struct OnScreenFpsMarker;
#[derive(Clone, Debug)]
pub struct OnScreenFpsPlugin(OnScreenFpsConfig);

fn fps_setup(
    mut commands:  Commands, 
    config: Res<OnScreenFpsConfig>, 
    asset_server: Res<AssetServer>
) {
    commands
        .spawn(TextBundle {
            text: Text::with_section(
                "FPS",
                TextStyle {
                    font: asset_server.load(config.font),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                TextAlignment::default()
            ),
            ..Default::default()
        })
        .with(OnScreenFpsMarker);
}

fn fps_update(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<OnScreenFpsMarker>>
) {
    if let Some(Some(fps)) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).map(|x| x.value()) {
        for mut text in query.iter_mut() {
            text.set(format!("{:<3.3}", fps));
        }
    }
}

impl OnScreenFpsPlugin {
    pub fn new(config: OnScreenFpsConfig) -> Self {
        Self(config)
    }
}

impl Plugin for OnScreenFpsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_resource(self.0.clone())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(fps_setup)
            .add_system(fps_update);
    }
}
