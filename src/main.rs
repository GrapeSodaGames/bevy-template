use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(Setup)
    .run();
}

// Components
#[derive(Component)]
struct Thing;

#[derive(Component)]
struct Position {x: f32, y:f32, z: f32}

// Resources
#[derive(Resource)]
struct TestTimer(Timer);

// Systems
fn add_test_entity(
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

    commands.spawn(camera);
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
    commands.spawn(cube);
}

fn read_test_entity(time: Res<Time>, mut timer: ResMut<TestTimer>, query: Query<&Position, With<Thing>>){
    if timer.0.tick(time.delta()).just_finished(){
        for pos in &query{
            println!("x = {}, y = {}, z = {} ", pos.x, pos.y, pos.z)
        }
    }
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

// Plugins
pub struct Setup;
impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_test_entity)
        .add_startup_system(add_camera)
        .add_startup_system(add_cube)
        .add_startup_system(add_light)
        .insert_resource(TestTimer(Timer:: from_seconds(2.0, TimerMode::Repeating)))
        .add_system(read_test_entity);
        
    }
}