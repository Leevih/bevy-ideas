use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Textures {
    pub minion: Handle<Image>,
    pub pit: Handle<Image>,
    pub stone: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Textures>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut textures: ResMut<Textures>, asset_server: Res<AssetServer>) {
    *textures = Textures {
        minion: asset_server.load("circle.png"),
        pit: asset_server.load("m.png"),
        stone: asset_server.load("s.png"),
    }
}
