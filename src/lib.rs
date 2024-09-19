use pyo3::prelude::*;
use bevy::prelude::*;

/// Initialize the Bevy app and run the game loop.
///
/// This function will be called from Python.
#[pyfunction]
fn run_bevy_app() -> PyResult<()> {
    // Initialize a Bevy App
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();

    Ok(())
}

// A simple component
#[derive(Component)]
struct HelloWorld;

// Setup system to add entities
fn setup(mut commands: Commands) {
    commands.spawn(HelloWorld);
}

// Update system to perform actions each frame
fn update(query: Query<&HelloWorld>) {
    for _entity in query.iter() {
        println!("Hello, Bevy from Python!");
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pybevy(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_bevy_app, m)?)?;
    Ok(())
}
