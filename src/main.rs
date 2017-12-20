#![feature(iterator_step_by)]
#![feature(alloc_system)] extern crate alloc_system;
extern crate rand;

use rand::{thread_rng, random, Rng};

const TARGET: &str = "methinks it is like a weasel";
const TARGET_LEN: usize = 28;
const MUT_PROB: f64 = 1f64 / TARGET_LEN as f64;

#[derive(Debug)]
struct EvoPheno {
    text: Vec<u8>,
    fitness: i32
}

impl EvoPheno {
    fn new(t: Vec<u8>) -> EvoPheno {
        let mut fitness: i32 = 0;
        let target_chars = TARGET.as_bytes();

        for i in 0..TARGET_LEN {
            if target_chars[i] == t[i] {
                fitness += 1;
            }
        }

        EvoPheno {
            text: t,
            fitness: fitness
        }
    }

    fn crossover(&self, other: &EvoPheno) -> EvoPheno {
        EvoPheno::new((0..TARGET_LEN).map(|i| if random::<f64>() < 0.5 {
            self.text[i]
        } else {
            other.text[i]
        }).collect())
    }

    fn mutate(&self) -> EvoPheno {
        EvoPheno::new((0..TARGET_LEN).map(|i| if random::<f64>() < MUT_PROB {
            thread_rng().gen_range(32, 127)
        } else {
            self.text[i]
        }).collect())
    }
}

fn tournament(population: &[EvoPheno]) -> (usize, usize) {
    let a_index = thread_rng().gen_range(0, population.len()) as usize;
    let b_index = thread_rng().gen_range(0, population.len()) as usize;

    if population[a_index].fitness < population[b_index].fitness {
        (a_index, b_index)
    } else {
        (b_index, a_index)
    }
}

fn run_algorithm(population_size: i32, crossover: bool) -> i32 {
    let mut iterations = 0;
    let mut population: Vec<EvoPheno> = (0..population_size).map(|_|
        EvoPheno::new(
            (0..TARGET_LEN).map(|_| thread_rng().gen_range(32, 127)).collect()
        )
    ).collect();

    loop {
        let parent1_index = tournament(&population).1;

        let child = if crossover {
            let parent2_index = tournament(&population).1;
            population[parent1_index].crossover(&population[parent2_index]).mutate()
        } else {
            population[parent1_index].mutate()
        };

        iterations += 1;

        if child.fitness == TARGET_LEN as i32 {
            return iterations;
        }

        let new_index = tournament(&population).0;
        std::mem::replace(&mut population[new_index], child);
    }
}

fn main() {
    for population_size in (50..500).step_by(50) {
        for crossover in &[true, false] {
            let mut results: Vec<i32> = (0..5).map(|_| run_algorithm(population_size, *crossover)).collect();
            results.sort();
            println!("{},{},{},{},{}", crossover, population_size, results[0], results[2], results[4]);
        }
    }
}
