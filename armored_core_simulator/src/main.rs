use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::*,
    utils::Duration,
};
use rand::random;

// Going to be simple Armored Core simulator
// 2 Pilots will customize their AC and will enter a simulated battle

#[derive(Component)]
struct Pilot {
    name: String,
    wins: i32,
}

// Affects the movement of the AC, in kilos
#[derive(Component)]
struct Weight {
    value: i32,
}

#[derive(Component)]
struct Armor {
    value: i32,
}

#[derive(Component)]
struct Damage {
    value: i32,
}

#[derive(Component)]
struct Firerate {
    value: i32,
}

#[derive(Component)]
struct Accuracy {
    value: i32,
}

#[derive(Component)]
struct Speed {
    value: i32,
}

// Weapon bundle
#[derive(Bundle)]
struct Weapon {
    damage: Damage,
    firerate: Firerate,
    accuracy: Accuracy,
}

// This resource holds info about the game
#[derive(Resource, Default)]
struct GameState {
    current_round: i32,
    surviving_pilot: Option<String>,
}

#[derive(Resource)]
struct GameRules {
    max_wins: i32,
}

fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!("Begin round {}", game_state.current_round)
}

fn score_check_system(
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
    query: Query<&Pilot>,
) {
    for pilot in &query {
        if pilot.wins == game_rules.max_wins {
            game_state.surviving_pilot = Some(pilot.name.clone());
        }
    }
}

fn simulation_over_system(
    game_rules: Res<GameRules>,
    game_state: Res<GameState>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Some(ref pilot) = game_state.surviving_pilot {
        println!("{pilot} survived.");
        app_exit_events.send(AppExit);
    }
}

fn startup_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
    commands.insert_resource(GameRules { max_wins: 2 });

    commands.spawn_batch(vec![
        (Pilot {
            name: "Blue Rain".to_string(),
            wins: 0,
        }),
        (Pilot {
            name: "Ninebreaker".to_string(),
            wins: 0,
        }),
    ]);
}

// Prints the stats of each AC
fn round_summary_system(
    pilot_query: Query<(&Pilot, &Children)>,
    body_part_query: Query<(&Armor, &Weight, &Speed)>,
) {
    for (pilot, children) in pilot_query.iter() {
        println!("Pilot {} with {} wins", pilot.name, pilot.wins);

        let mut armor: i32 = 0;
        for &body_part in children {
            let part_result = body_part_query.get(body_part);
            if let Ok(part) = part_result {
                armor += part.0.value;
            }
        }
        println!("Armor: {}", armor);
    }
}

// We have pilot entities, and they will have their own AC.
// Each pilot entity will have multiple children entities which will
// consist of the parts of an AC, such as the head, core, arms, legs, and weapon
fn spawn_ac_one(mut commands: Commands, query: Query<(Entity, &Pilot), (Without<Children>)>) {
    for (e, pilot) in query.iter() {
        if pilot.name == "Blue Rain" {
            let core = commands
                .spawn((Armor { value: 3000 }, Weight { value: 1000 }))
                .id();
            let head = commands
                .spawn((
                    Armor { value: 500 },
                    Weight { value: 500 },
                    Accuracy { value: 1000 },
                ))
                .id();
            let arms = commands
                .spawn((Armor { value: 1000 }, Weight { value: 1000 }))
                .id();
            let legs = commands
                .spawn((
                    Armor { value: 1500 },
                    Weight { value: 1500 },
                    Speed { value: 4500 },
                ))
                .id();
            let weapon = commands
                .spawn((
                    Weapon {
                        damage: Damage { value: 100 },
                        firerate: Firerate { value: 100 },
                        accuracy: Accuracy { value: 1000 },
                    },
                    Weight { value: 1000 },
                ))
                .id();
            commands
                .entity(e)
                .push_children(&[core, head, arms, legs, weapon]);
        };
    }
}

fn spawn_ac_two(mut commands: Commands, query: Query<(Entity, &Pilot), (Without<Children>)>) {
    for (e, pilot) in query.iter() {
        if pilot.name == "Ninebreaker" {
            let core = commands
                .spawn((Armor { value: 5000 }, Weight { value: 5000 }))
                .id();
            let head = commands
                .spawn((
                    Armor { value: 700 },
                    Weight { value: 700 },
                    Accuracy { value: 500 },
                ))
                .id();
            let arms = commands
                .spawn((Armor { value: 2000 }, Weight { value: 2000 }))
                .id();
            let legs = commands
                .spawn((
                    Armor { value: 3500 },
                    Weight { value: 3500 },
                    Speed { value: 7500 },
                ))
                .id();
            let weapon = commands
                .spawn((
                    Weapon {
                        damage: Damage { value: 750 },
                        firerate: Firerate { value: 17 },
                        accuracy: Accuracy { value: 1000 },
                    },
                    Weight { value: 1000 },
                ))
                .id();
            commands
                .entity(e)
                .push_children(&[core, head, arms, legs, weapon]);
        };
    }
}

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_startup_system(startup_system)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_system(new_round_system)
        .add_system(spawn_ac_one)
        .add_system(spawn_ac_two)
        .add_system(score_check_system)
        .add_system(round_summary_system)
        .add_system(simulation_over_system)
        .run();
}
