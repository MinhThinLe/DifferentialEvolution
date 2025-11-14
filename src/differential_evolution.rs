pub struct DEConfig {
    population_size: u32,
    crossover_compatibility: f32,
    differential_weight: f32,
}

struct Agent {
    data: Vec<f32>,
}

pub struct DifferentialEvolution {
    agents: Vec<Agent>,
    config: DEConfig,
}

impl Default for DEConfig {
    fn default() -> Self {
        Self { population_size: 100, crossover_compatibility: 0.9, differential_weight: 0.8 }
    }
}

impl DEConfig {
    fn new() -> Self {
        Self::default()
    }

    fn set_population(mut self, population_size: u32) -> Self {
        self.population_size = population_size;
        self
    }

    fn set_crossover_compatibility(mut self, crossover_compatibility: f32) -> Self {
        self.crossover_compatibility = crossover_compatibility;
        self
    }

    fn set_differential_weight(mut self, differential_weight: f32) -> Self {
        self.differential_weight = differential_weight;
        self
    }
}
