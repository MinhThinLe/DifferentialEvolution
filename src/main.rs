mod differential_evolution;

use differential_evolution::*;

fn main() {
    let config = DEConfig::new()
        .set_population(200)
        .set_differential_weight(0.85)
        .set_crossover_compatibility(0.85);
    let differential_evolution = DifferentialEvolution::new(config);
}
