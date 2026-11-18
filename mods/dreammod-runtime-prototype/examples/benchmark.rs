//! Coarse microbenchmark only. Results from CI hosts are not device guarantees.
use dreammod_runtime_prototype::RhaiPrototype;
use std::{hint::black_box, time::Instant};

const ITEMS: usize = 10_000;
const ROUNDS: usize = 50;

fn main() {
    let positions = vec![1.0_f64; ITEMS];
    let velocities = vec![0.25_f64; ITEMS];

    let program = RhaiPrototype::parse(
        r#"
        fn physics_step(positions, velocities, delta_seconds) {
            let next = [];
            for i in 0..positions.len() {
                next.push(positions[i] + velocities[i] * delta_seconds);
            }
            next
        }
        "#,
    )
    .expect("valid benchmark script");

    let start = Instant::now();
    for _ in 0..ROUNDS {
        let output = program
            .call_physics_batch(
                black_box(positions.clone()),
                black_box(velocities.clone()),
                0.05,
            )
            .expect("script evaluation");
        assert_eq!(output.len(), ITEMS);
        let output = black_box(output);
        black_box(output.iter().sum::<f64>());
    }
    let interpreted = start.elapsed();

    let start = Instant::now();
    for _ in 0..ROUNDS {
        let positions_input = black_box(positions.clone());
        let velocities_input = black_box(velocities.clone());
        let output: Vec<f64> = positions_input
            .iter()
            .zip(&velocities_input)
            .map(|(position, velocity)| position + velocity * 0.05)
            .collect();
        assert_eq!(output.len(), ITEMS);
        let output = black_box(output);
        black_box(output.iter().sum::<f64>());
    }
    let native = start.elapsed();

    println!("items_per_round={ITEMS}");
    println!("rounds={ROUNDS}");
    println!("rhai_ast_interpreter_total={interpreted:?}");
    println!("rust_native_total={native:?}");
    println!(
        "ratio_interpreted_over_native={:.2}",
        interpreted.as_secs_f64() / native.as_secs_f64().max(f64::MIN_POSITIVE)
    );
}
