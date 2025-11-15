mod differential_evolution;

use differential_evolution::*;

fn main() {
    let config = DEConfig::new()
        .set_population(250)
        .set_differential_weight(0.8)
        .set_crossover_compatibility(0.9)
        .set_seed(1234);
    let mut differential_evolution = DifferentialEvolution::new(config);

    for _ in 0..1000 {
        differential_evolution.step();
    }
    println!("{:?}", differential_evolution.get_fittest_candidate());
}
