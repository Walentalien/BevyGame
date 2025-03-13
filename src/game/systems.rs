use bevy::prelude::*;
use crate::game::SimulationState;
use bevy::prelude::NextState;
pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused");

        }
        if *simulation_state.get()== SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
    }
}