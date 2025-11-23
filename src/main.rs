mod differential_evolution;

use differential_evolution::*;

fn main() {
    let config = DEConfig::new()
        .set_population(250)
        .set_differential_weight(0.8)
        .set_crossover_compatibility(0.9)
        .set_dimensions(3)
        .set_seed(1235);
    let mut differential_evolution = DifferentialEvolution::new(config);
    
    for _generation in 0..50 {
        println!("{}", differential_evolution.get_fittest_candidate().unwrap());
        differential_evolution.step();
    }
}
