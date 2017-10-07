#![feature(iterator_step_by)]
#![feature(alloc_system)] extern crate alloc_system;
extern crate rand;

use rand::{thread_rng, Rng};

const TARGET: &str = "methinks it is like a weasel";
const TARGET_LEN: usize = 28;

#[derive(Clone, Debug)]
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

    fn new_random() -> EvoPheno {
        EvoPheno::new((0..TARGET_LEN).map(|_| thread_rng().gen_range(32, 127)).collect())
    }

    fn crossover(&self, other: &EvoPheno) -> EvoPheno {
        EvoPheno::new((0..TARGET_LEN).map(|i| if rand::random::<f64>() < 0.5 {
            self.text[i]
        } else {
            other.text[i]
        }).collect())
    }

    fn mutate(&self) -> EvoPheno {
        EvoPheno::new((0..TARGET_LEN).map(|i| if rand::random::<f64>() < (1f64 / TARGET_LEN as f64) {
            thread_rng().gen_range(32, 127)
        } else {
            self.text[i]
        }).collect())
    }
}

fn run_algorithm(population_size: i32, crossover: bool) -> i32 {
    let mut population: Vec<EvoPheno> = (0..population_size).map(|_| EvoPheno::new_random()).collect();
    let mut iterations = 0;

    loop {
        let a_index = thread_rng().gen_range(0, population_size) as usize;
        let b_index = thread_rng().gen_range(0, population_size) as usize;
        let parent1_index = if population[a_index].fitness > population[b_index].fitness {
            a_index
        } else {
            b_index
        };

        let child = if crossover {
            let c_index = thread_rng().gen_range(0, population_size) as usize;
            let d_index = thread_rng().gen_range(0, population_size) as usize;
            let parent2_index = if population[c_index].fitness > population[d_index].fitness {
                c_index
            } else {
                d_index
            };

            let cross_child = population[parent1_index].crossover(&population[parent2_index]);
            cross_child.mutate()
        } else {
            population[parent1_index].mutate()
        };

        iterations += 1;

        if child.fitness == TARGET_LEN as i32 {
            return iterations;
        }

        let c_index = thread_rng().gen_range(0, population_size) as usize;
        let d_index = thread_rng().gen_range(0, population_size) as usize;
        let mut new_population = population;

        if new_population[c_index].fitness > new_population[d_index].fitness {
            std::mem::replace(&mut new_population[d_index], child);
        } else {
            std::mem::replace(&mut new_population[c_index], child);
        }

        population = new_population;
    }
}

fn main() {
    run_algorithm(500, true);

/*    for population_size in (50..500).step_by(50) {
        for crossover in vec![true, false] {
            print!("{},{}", crossover, population_size);
            let mut results = Vec::new();

            for _ in 0..5 {
                results.push(run_algorithm(population_size, crossover));
            }

            results.sort();

            println!(",{},{},{}", results[0], results[2], results[4]);
        }
    }*/
}
