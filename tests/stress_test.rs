/// Stress test: 10,000 NPC tick performance benchmark.
///
/// Run with: cargo test --release -- --nocapture stress
///
/// This validates the core claim: 10,000 NPCs can be processed
/// in a single tick without exceeding the 200ms budget.

use std::collections::HashMap;
use std::time::Instant;

use l1j_rust::ecs::components::npc::NpcTemplate;
use l1j_rust::ecs::components::position::Position;
use l1j_rust::ecs::game_engine::GameWorld;

fn make_monster_template(npc_id: i32) -> NpcTemplate {
    NpcTemplate {
        npc_id,
        name: format!("Monster_{}", npc_id),
        nameid: format!("monster_{}", npc_id),
        impl_type: "L1Monster".to_string(),
        gfxid: 100 + npc_id,
        level: 10,
        hp: 500,
        mp: 100,
        ac: 5,
        str_stat: 15, con_stat: 12, dex_stat: 14,
        wis_stat: 10, int_stat: 10, mr: 20,
        exp: 1000, lawful: -100,
        size: "small".to_string(),
        undead: 0, poison_atk: 0, paralysis_atk: 0,
        agro: true, agrososc: true, agrocoi: false,
        family: 0, agrofamily: 0, pickup_item: false,
        brave_speed: 0, passispeed: 640, atkspeed: 1020,
        atk_magic_speed: 0, tamable: false, teleport: false,
        doppel: false, hpr_interval: 12000, hpr: 3,
        mpr_interval: 12000, mpr: 2, ranged: 0, light_size: 0,
        change_head: false, damage_reduction: 0, hard: false,
        karma: 0, transform_id: 0, transform_gfxid: 0,
        cant_resurrect: false,
    }
}

#[test]
fn stress_10000_npcs_100_ticks() {
    println!("\n=== STRESS TEST: 10,000 NPCs x 100 Ticks ===\n");

    let mut templates = HashMap::new();
    for i in 0..10 {
        templates.insert(45000 + i, make_monster_template(45000 + i));
    }

    let mut world = GameWorld::new(templates);

    let spawn_start = Instant::now();
    for i in 0..10_000i32 {
        let template_id = 45000 + (i % 10);
        let x = 32000 + (i % 200);
        let y = 32000 + (i / 200);
        world.spawn_npc(template_id, x, y, 4);
    }
    let spawn_time = spawn_start.elapsed();
    println!("Spawned 10,000 NPCs in {:?}", spawn_time);
    println!("Grid: {} objects in {} regions",
        world.grid.total_objects(), world.grid.active_regions());

    // Place 5 players at different spots
    world.player_positions.insert(90001, Position::new(32050, 32010, 4));
    world.player_positions.insert(90002, Position::new(32100, 32020, 4));
    world.player_positions.insert(90003, Position::new(32150, 32030, 4));
    world.player_positions.insert(90004, Position::new(32050, 32040, 4));
    world.player_positions.insert(90005, Position::new(32100, 32040, 4));

    let mut total_movements = 0usize;
    let mut tick_times = Vec::with_capacity(100);

    let total_start = Instant::now();
    for tick in 0..100 {
        let tick_start = Instant::now();
        let movements = world.tick(30);
        let tick_time = tick_start.elapsed();

        total_movements += movements.len();
        tick_times.push(tick_time);

        if tick < 3 || tick == 99 {
            println!("Tick {:3}: {:5} movements, {:?}",
                tick, movements.len(), tick_time);
        }
    }
    let total_time = total_start.elapsed();

    let avg_tick = total_time / 100;
    let max_tick = tick_times.iter().max().unwrap();
    let min_tick = tick_times.iter().min().unwrap();

    println!("\n--- Results ---");
    println!("Total time for 100 ticks: {:?}", total_time);
    println!("Average tick time: {:?}", avg_tick);
    println!("Min tick time: {:?}", min_tick);
    println!("Max tick time: {:?}", max_tick);
    println!("Total movements: {}", total_movements);
    println!("Budget per tick: 200ms");
    println!("Status: {}", if max_tick.as_millis() < 200 { "PASS !!!" } else { "FAIL" });

    assert!(
        max_tick.as_millis() < 200,
        "Max tick time {:?} exceeds 200ms budget!",
        max_tick
    );
    assert!(total_movements > 0, "No NPCs moved at all!");
}

#[test]
fn stress_visibility_query_10000() {
    println!("\n=== STRESS TEST: Visibility Query with 10,000 Objects ===\n");

    let mut templates = HashMap::new();
    templates.insert(45000, make_monster_template(45000));

    let mut world = GameWorld::new(templates);

    for i in 0..10_000i32 {
        let x = 32000 + (i % 200);
        let y = 32000 + (i / 200);
        world.spawn_npc(45000, x, y, 4);
    }

    let start = Instant::now();
    let mut total_visible = 0usize;
    for i in 0..10_000i32 {
        let x = 32000 + (i % 200);
        let y = 32000 + (i / 200);
        let nearby = world.grid.get_nearby(4, x, y);
        total_visible += nearby.len();
    }
    let elapsed = start.elapsed();

    println!("10,000 visibility queries in {:?}", elapsed);
    println!("Average per query: {:?}", elapsed / 10_000);
    println!("Total visible objects returned: {}", total_visible);

    assert!(
        elapsed.as_millis() < 1000,
        "10,000 queries took {:?}, too slow!",
        elapsed
    );
}
