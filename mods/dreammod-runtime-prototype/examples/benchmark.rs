//! Coarse microbenchmark only. Results from CI hosts are not device guarantees.
use dreammod_runtime_prototype::RhaiPrototype;
use std::time::Instant;

const ITEMS: usize = 10_000;
const ROUNDS: usize = 50;

fn main() {
    let positions = vec![1.0_f64; ITEMS];
    let velocities = vec![0.25_f64; ITEMS];

    let program = RhaiPrototype::parse(
        r#"
        fn physics_step(positions, velocities, delta_seconds) {
            let next = [];
            for i in 0..positions.len {
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
            .call_physics_batch(positions.clone(), velocities.clone(), 0.05)
            .expect("script evaluation");
        assert_eq!(output.len(), ITEMS);
    }
    let interpreted = start.elapsed();

    let start = Instant::now();
    for _ in 0..ROUNDS {
        let output: Vec<f64> = positions
            .iter()
            .zip(&velocities)
            .map(|(position, velocity)| position + velocity * 0.05)
            .collect();
        assert_eq!(output.len(), ITEMS);
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
