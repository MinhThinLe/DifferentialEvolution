use rand::prelude::*;
use std::f32::consts::{E, PI};

#[derive(Debug)]
pub struct DEConfig {
    population_size: usize,
    crossover_compatibility: f32,
    differential_weight: f32,
    dimensions: u32,
    seed: u64,
}

#[derive(Clone)]
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

    // https://en.wikipedia.org/wiki/Differential_evolution#Algorithm
    fn mutate(&mut self, references: &[Agent], rng: &mut SmallRng, de_config: &DEConfig) {
        let r_index = rng.random_range(0..self.data.len()); // IDK how to name this, ask Wikipedia
        for (index, agent_data) in self.data.iter_mut().enumerate() {
            if index != r_index && rng.random::<f32>() > de_config.crossover_compatibility {
                continue;
            }
            // y_i = a_i + F * (b_i - c_i)
            *agent_data = references[0].data[index]
                + (de_config.differential_weight
                    * (references[1].data[index] - references[2].data[index]));
        }
    }

    fn fitness(&self) -> f32 {
        // ackley_function(self.data[0], self.data[1])
        rastrigin_function(&self.data)
    }
}

impl Default for DEConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            crossover_compatibility: 0.9,
            differential_weight: 0.8,
            dimensions: 2,
            seed: 0,
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

    pub fn set_dimensions(mut self, dimensions: u32) -> Self {
        self.dimensions = dimensions;
        self
    }
}

impl DifferentialEvolution {
    pub fn new(config: DEConfig) -> Self {
        let mut rng = SmallRng::seed_from_u64(config.seed);
        let mut agents = Vec::with_capacity(config.population_size);

        for _ in 0..config.population_size {
            let agent_data: Vec<f32> = (0..config.dimensions).map(|_| rng.random()).collect();
            agents.push(Agent::with_data(&agent_data));
        }

        Self {
            config: config,
            agents,
            randomizer: rng,
        }
    }

    pub fn get_fittest_candidate(&self) -> Option<f32> {
        self.agents
            .iter()
            .map(|agent| agent.fitness())
            .reduce(f32::max)
    }

    pub fn step(&mut self) {
        for index in 0..self.agents.len() {
            let indices =
                gen_range_unique(&mut self.randomizer, index, self.config.population_size);
            let mut candidate = self.agents[index].clone();
            let baseline = candidate.fitness();
            let references = [
                self.agents[indices[0]].clone(),
                self.agents[indices[1]].clone(),
                self.agents[indices[2]].clone(),
            ];

            candidate.mutate(&references, &mut self.randomizer, &self.config);

            if candidate.fitness() > baseline {
                continue;
            }

            *self.agents.get_mut(index).unwrap() = candidate
        }
    }
}

// https://en.wikipedia.org/wiki/Test_functions_for_optimization#Test_functions_for_constrained_optimization
#[allow(dead_code)]
fn ackley_function(x: f32, y: f32) -> f32 {
    -20.0 * E.powf(-0.2 * (0.5 * (x * x + y * y)).sqrt())
        - E.powf(0.5 * ((2.0 * PI * x).cos() + (2.0 * PI * y).cos()))
        + E
        + 20.0
}

#[allow(dead_code)]
fn rastrigin_function(array: &[f32]) -> f32 {
    const A: f32 = 10.0;
    let sum: f32 = array.iter().map(|x| x * x - A * (2.0 * PI * x).cos()).sum();
    A * array.len() as f32 + sum
}

fn gen_range_unique(rng: &mut SmallRng, exclude: usize, max: usize) -> [usize; 3] {
    loop {
        let candidate: Vec<usize> = (0..3).map(|_| rng.random_range(0..max)).collect();

        // Check for collisions within the generated array
        for element in candidate.iter() {
            let count = candidate
                .iter()
                .map(|elem| (elem == element) as u8)
                .sum::<u8>();
            if count > 1 {
                continue;
            }
        }
        if candidate.contains(&exclude) {
            continue;
        }

        return [candidate[0], candidate[1], candidate[2]];
    }
}
