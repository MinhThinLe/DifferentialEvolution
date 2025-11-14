use rand::prelude::*;

pub struct DEConfig {
    population_size: usize,
    crossover_compatibility: f32,
    differential_weight: f32,
    seed: u64,
}

struct Agent {
    data: Vec<f32>,
}

pub struct DifferentialEvolution {
    agents: Vec<Agent>,
    config: DEConfig,
    randomizer: SmallRng,
}

impl Agent {
    fn with_data(data: &[f32]) -> Self {
        Agent {
            data: data.to_vec(),
        }
    }
}

impl Default for DEConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            crossover_compatibility: 0.9,
            differential_weight: 0.8,
            seed: 0
        }
    }
}

impl DEConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_population(mut self, population_size: usize) -> Self {
        self.population_size = population_size;
        self
    }

    pub fn set_crossover_compatibility(mut self, crossover_compatibility: f32) -> Self {
        self.crossover_compatibility = crossover_compatibility;
        self
    }

    pub fn set_differential_weight(mut self, differential_weight: f32) -> Self {
        self.differential_weight = differential_weight;
        self
    }

    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
}

impl DifferentialEvolution {
    pub fn new(config: DEConfig) -> Self {
        let mut rng = SmallRng::seed_from_u64(config.seed);
        let mut agents = Vec::with_capacity(config.population_size);

        for _ in 0..config.population_size {
            let agent = Agent::with_data(&vec![rng.random::<f32>(), rng.random::<f32>()]);
            agents.push(agent);
        }

        Self {
            config: config,
            agents,
            randomizer: rng
        }
    }

}
