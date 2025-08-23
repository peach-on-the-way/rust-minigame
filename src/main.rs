use prelude::*;

mod prelude;

mod ecs;

mod collision;
mod damage;
mod draw;
mod enemy;
mod input;
mod player;
mod sprite;
mod timer;
mod ui;

mod utils;

fn main() {
    let mut stdout = io::stdout();

    if !terminal::supports_keyboard_enhancement().expect("Check keyboard enhancement") {
        println!("Your terminal does not support keyboard enhancement! Please change your terminal.");
        return;
    }

    // Setup terminal
    terminal::enable_raw_mode().expect("Enable raw mode");
    execute!(&mut stdout, terminal::EnterAlternateScreen, terminal::DisableLineWrap, event::DisableBracketedPaste, event::DisableMouseCapture, cursor::Hide, cursor::SavePosition, event::PushKeyboardEnhancementFlags(event::KeyboardEnhancementFlags::all()),).expect("Initialize terminal");

    // Entities
    let mut entities: Entities = Default::default();

    // Components
    let mut hps: Components<Health> = Default::default();
    let mut max_hps: Components<Health> = Default::default();
    let mut positions: Components<Vec2i32> = Default::default();
    let mut sprites: Components<Sprite> = Default::default();
    let mut move_timers: Components<Timer> = Default::default();
    let mut draw_infos: Components<DrawInfo> = Default::default();
    let mut damaged_timers: Components<Timer> = Default::default();
    let mut damaged_colors: Components<Color> = Default::default();

    // Events
    let mut damage_events: Events<Damage> = Default::default();
    let mut kill_events: Events<Kill> = Default::default();
    let mut spawn_draw_events: Events<Draw> = Default::default();

    // Resources
    let mut inputs: Inputs = Default::default();
    let mut enemies: HashSet<Entity> = Default::default();
    let mut collider_grid: ColliderGrid = Default::default();
    let mut score: i32 = 0;

    // Setup
    let player_id = entities.spawn();
    sprites.insert(&entities, player_id, Sprite { char: '@', ..Default::default() }).unwrap();
    positions.insert(&entities, player_id, (0, 0)).unwrap();
    hps.insert(&entities, player_id, 20).unwrap();
    max_hps.insert(&entities, player_id, 20).unwrap();
    damaged_timers.insert(&entities, player_id, Timer::new_ended(Duration::from_millis(200))).unwrap();
    damaged_colors.insert(&entities, player_id, Color::Red).unwrap();
    let player = Player { id: player_id, primary_weapon: Weapon::Stick };

    let camera_id = entities.spawn();
    positions.insert(&entities, camera_id, (0, 0)).unwrap();

    let arena_extend = (30i32, 10i32);
    for _ in 0..arena_extend.0 * 2 + 1 {
        collider_grid.0.push(vec![None; (arena_extend.1 * 2 + 1) as usize]);
    }

    execute!(&mut stdout, terminal::SetSize((arena_extend.0 * 2 + 1 + 50) as u16, (arena_extend.1 * 2 + 1 + 50) as u16)).unwrap();

    let mut player_dead = false;

    let mut move_timer = Timer::new(Duration::from_millis(50));
    let mut spawn_enemy_timer = Timer::new(Duration::from_secs(3));
    let mut weapon_timer = Timer::new(player.primary_weapon.base_delay());

    let mut prev_instant = std::time::Instant::now();
    loop {
        // Event intialization ========================================================================================
        damage_events.clear();
        kill_events.clear();
        spawn_draw_events.clear();

        // Delta calculation ==========================================================================================
        let next_instant = std::time::Instant::now();
        let delta = next_instant - prev_instant;
        prev_instant = std::time::Instant::now();

        // Systems ====================================================================================================

        input_system(&mut inputs);

        if inputs.pressed.contains(&KeyCode::Esc) {
            break;
        }

        if !player_dead {
            move_timer.current += delta;
            spawn_enemy_timer.current += delta;
            weapon_timer.current += delta;
            timer_system(delta, &entities, &mut move_timers);
            timer_system(delta, &entities, &mut damaged_timers);

            if spawn_enemy_timer.finished() {
                spawn_enemy_system(&arena_extend, &mut enemies, &mut collider_grid, &mut entities, &mut sprites, &mut positions, &mut hps, &mut move_timers, &mut damaged_timers, &mut damaged_colors);
                spawn_enemy_timer.reset();
            }

            player_movement_system(&mut move_timer, &arena_extend, player_id, &inputs, &mut collider_grid, &mut entities, &mut positions);
            player_weapon_system(&arena_extend, &player, &mut weapon_timer, &collider_grid, &mut spawn_draw_events, &mut damage_events, &entities, &inputs, &positions);

            enemy_follow_system(&arena_extend, &player, &enemies, &mut collider_grid, &mut damage_events, &entities, &mut positions, &mut move_timers);
            damage_system(&mut stdout, &damage_events, &mut kill_events, &entities, &mut hps, &mut damaged_timers);
            enemy_killed_system(&arena_extend, &kill_events, &mut collider_grid, &mut score, &mut enemies, &mut entities, &positions);
            player_killed_system(&kill_events, &mut player_dead, &player);
        }

        spawn_draw_system(&spawn_draw_events, &mut entities, &mut positions, &mut draw_infos);

        // Rendering  ----------------------------------------------------------------------------------------------

        let mut stdout = stdout.lock();
        queue!(&mut stdout, terminal::BeginSynchronizedUpdate, terminal::Clear(terminal::ClearType::Purge)).unwrap();

        draw_system(&mut stdout, delta, camera_id, &mut entities, &positions, &mut draw_infos);
        sprite_system(&mut stdout, camera_id, &entities, &positions, &sprites, &damaged_timers, &damaged_colors);
        visualize_arena_wall_system(&mut stdout, &arena_extend, camera_id, &entities, &positions);
        hud_system(&mut stdout, &arena_extend, &score, &player, &entities, &hps, &max_hps);
        if player_dead {
            display_end_screen_system(&mut stdout, &score);
        }

        queue!(&mut stdout, terminal::EndSynchronizedUpdate, cursor::RestorePosition).unwrap();
        stdout.flush().expect("Flush stdout");
    }

    // Clean up
    terminal::disable_raw_mode().expect("Disable raw mode");
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show, event::PopKeyboardEnhancementFlags).expect("Cleanup terminal");
}
