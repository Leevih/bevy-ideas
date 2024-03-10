use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct WorldLastClicked{
    pub value: Vec2
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;



pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App){
        app.init_resource::<WorldLastClicked>().add_systems(Startup, setup).add_systems(Update, my_cursor_system);
    }
}

fn setup(mut commands: Commands){
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn my_cursor_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut last_clicked_world_coordinates: ResMut<WorldLastClicked>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = q_camera.single();
        
        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();
        
        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            last_clicked_world_coordinates.value = world_position;
            
        }
    }
}