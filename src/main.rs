mod differential_evolution;

use differential_evolution::*;
use rand::Rng;
use std::env;

const RUNS: usize = 30;
const GENERATIONS: usize = 100;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 4 {
        println!("Insufficient arguments");
        println!("Usage: <cmd> <population> <weight> <cross-compat>");
        return;
    }
    let population = arguments[1].parse().expect("Invalid population");
    let differential_weight = arguments[2].parse().expect("Invalid weight");
    let cross_compatibility = arguments[3].parse().expect("Invalid compatibility");

    let mut data: Vec<Vec<f32>> = Vec::with_capacity(RUNS);
    let mut rng = rand::rng();
    for _run in 0..RUNS {
        let mut current: Vec<f32> = Vec::with_capacity(GENERATIONS);
        let config = DEConfig::new()
            .set_population(population)
            .set_differential_weight(differential_weight)
            .set_crossover_compatibility(cross_compatibility)
            .set_dimensions(3)
            .set_seed(rng.random())
            // jDE stuff
            .set_cross_compat_mutate_probability(0.1)
            .set_weight_mutate_probability(0.1)
            .set_lower_weight(0.1)
            .set_upper_weight(0.1);
        let mut differential_evolution = DifferentialEvolution::new(config);
        for _generation in 0..GENERATIONS {
            let best_fitness = differential_evolution.get_fittest_candidate().unwrap().fitness();
            current.push(best_fitness);
            differential_evolution.step();
        }
        data.push(current);
    }
    for generation in 0..GENERATIONS {
        let mut generation_average = 0.0;
        for run in 0..RUNS {
            generation_average += data[run][generation];
        }
        generation_average /= RUNS as f32;
        println!("{generation_average}")
    }
}
