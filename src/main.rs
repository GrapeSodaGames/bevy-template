use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::{CursorGrabMode, PresentMode, PrimaryWindow, Cursor}, ecs::schedule::OrElse,
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: bevy::window::WindowMode::BorderlessFullscreen, 
                title: "Bevy GameDev Template".to_string(),  
                resizable: false, 
                ..default()
            }),
            ..Default::default()
    }))
    .add_plugin(DebugStuff)
    .add_plugin(FlyCameraPlugin)
    .add_plugin(Setup)
    .run();
}

// Components
#[derive(Component)]
struct Thing;

// Resources
#[derive(Resource)]
struct TestTimer(Timer);

#[derive(Resource)]
struct PlayerInputs {
    mouse_locked: bool,

}

// Systems
fn add_plane(
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    //plane
    let plane = PbrBundle{
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    };
    commands.spawn(plane);
}

fn add_camera(mut commands : Commands){
    let camera : Camera3dBundle = Camera3dBundle {
        transform: Transform::from_xyz(-2.0,2.5,5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera)
    .insert(FlyCamera::default());
}

fn add_cube(
    mut commands : Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let cube = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
        material: materials.add(Color::rgb(0.8, 0.7, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };
    commands.spawn((Thing, cube));
}

fn add_light(
    mut commands: Commands
){
    let light = PointLightBundle{
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default() 
        },
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(light);
}

fn get_things_pos(time: Res<Time>, mut timer: ResMut<TestTimer>, query: Query<&Transform, With<Thing>>){
    if timer.0.tick(time.delta()).just_finished(){
        for pos in &query{
            println!("{} ", pos.translation)
        }
    }
}



fn toggle_vsync(mut window_query: Query<&mut Window, With<PrimaryWindow>>,){
    if let Ok(mut window) = window_query.get_single_mut(){
        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync){
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode )
    }
}

fn toggle_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>, inputs: Res<PlayerInputs>) {
    if let Ok(mut window) = window_query.get_single_mut(){
        info!("{}",window.title);
        if inputs.mouse_locked {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        } else {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
        
        
    } else {
        info!("No primary window found.");
    }
}

fn handle_input(keys: Res<Input<KeyCode>>, mut opt: ResMut<PlayerInputs>){
    if keys.just_pressed(KeyCode::M){
        opt.mouse_locked = !opt.mouse_locked;
    }
}
    

// Plugins
pub struct Setup;
impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_plane)
        .add_startup_system(add_camera)
        .add_startup_system(add_cube)
        .add_startup_system(add_light)
        .insert_resource(PlayerInputs {
            mouse_locked: false
        })
        .insert_resource(TestTimer(Timer:: from_seconds(2.0, TimerMode::Repeating)))
        .add_system(handle_input)
        .add_system(toggle_cursor)
        .add_system(get_things_pos)
        ;
        
    }
}
pub struct DebugStuff;
impl Plugin for DebugStuff{
    fn build(&self, app: &mut App){
        app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(toggle_vsync);

    }
}
// pub struct GraphicsStuff;
// impl Plugin for GraphicsStuff{
//     fn build(&self, app: &mut App) {
//         app.add_system(toggle_vsync)
//         .run();
//     }
// }