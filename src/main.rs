mod differential_evolution;

use differential_evolution::*;

fn main() {
    let config = DEConfig::new()
        .set_population(250)
        .set_differential_weight(0.5)
        .set_crossover_compatibility(0.75)
        .set_seed(1235);
    let mut differential_evolution = DifferentialEvolution::new(config);

    println!("Before: {:?}", differential_evolution.get_fittest_candidate().unwrap());
    for _generation in 0..1000 {
        differential_evolution.step();
    }
    println!("After: {:?}", differential_evolution.get_fittest_candidate().unwrap());
}
