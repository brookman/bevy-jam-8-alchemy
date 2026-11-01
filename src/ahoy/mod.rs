use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_framepace::FramepacePlugin;

mod ahoy_debug;
mod camera;
mod input;
mod player;
mod scene;
mod stable_ground;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        AhoyPlugins::default(),
        FramepacePlugin,
        //MipmapGeneratorPlugin,
        scene::plugin,
        input::plugin,
        camera::plugin,
        player::plugin,
        stable_ground::plugin,
        ahoy_debug::plugin,
    ));
}
