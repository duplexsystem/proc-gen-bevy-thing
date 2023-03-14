use bevy::prelude::{App, Camera3dBundle, Commands, PluginGroup, Transform, Vec3};
use bevy::render::RenderPlugin;
use bevy::DefaultPlugins;
use bevy_rapier3d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use bevy_rust_gpu::RustGpuPlugin;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // Configure the render plugin with RustGpuPlugin's recommended WgpuSettings
            RenderPlugin {
                wgpu_settings: RustGpuPlugin::wgpu_settings(),
            },
        ))
        .add_plugin(RustGpuPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
