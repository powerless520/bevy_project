use bevy::{prelude::*};

fn hello_world() {
    println!("hello world!")
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time:Res<Time>,mut timer:ResMut<GreetTimer>,query:Query<&Name,With<Person>>) {
    // update out timer with the time elapsed since the last update
    // if that caused the timer to finish,we say hello to everyone

    for name in &query {
        println!("hello {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        // .add_startup_system(add_people)
        // .add_system(hello_world)
        // .add_system(greet_people)
        .run();
}