// use std::time::Instant;
use rand::prelude::*;
use thousands::Separable;

use crate::board::{ArrayPosition, PATTERN_NUM, PATTERN_WEIGHTS};
use crate::solve::{solve_top, NODE_COUNT};
use crate::types::*;

#[derive(Clone)]
struct Vector {
    vec: Vec<i32>,
    fitness: usize,
}

impl Vector {
    pub fn new() -> Self {
        let vec = vec![0; PATTERN_NUM];
        let fitness = 0;
        Vector { vec, fitness }
    }

    fn rand() -> Self {
        let mut vec = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..PATTERN_NUM {
            let value: i32 = rng.gen_range(0..100);
            vec.push(value);
        }
        let fitness = 0;
        Vector { vec, fitness }
    }


    fn from_weights(weights: [i32; PATTERN_NUM]) -> Self {
        let mut vec = vec![];
        for w in weights {
            vec.push(w);
        }
        let fitness = 0;
        Vector { vec, fitness }
    }

    fn crossover(self: &Self, other: &Self) -> (Self, Self) {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..PATTERN_NUM - 1);
        println!("crossover point = {}", index);
        let mut v1 = Vector::new();
        let mut v2 = Vector::new();
        for i in 0..PATTERN_NUM {
            if i < index {
                v1.vec[i] = self.vec[i];
                v2.vec[i] = other.vec[i];
            } else {
                v1.vec[i] = other.vec[i];
                v2.vec[i] = self.vec[i];
            }
        }
        (v1, v2)
    }

    fn crossover2(self: &Self, other: &Self) -> (Self, Self) {
        let mut rng = thread_rng();

        let mut v1 = Vector::new();
        let mut v2 = Vector::new();

        for i in 0..PATTERN_NUM {
            if rng.gen_range(0.0..1.0) < 0.5 {
                v1.vec[i] = self.vec[i];
                v2.vec[i] = other.vec[i];
            } else {
                v1.vec[i] = other.vec[i];
                v2.vec[i] = self.vec[i];
            }
        }
        (v1, v2)
    }

    pub fn evaluate(self: &mut Self) {
        for i in 0..PATTERN_NUM {
            unsafe {
                PATTERN_WEIGHTS[i] = self.vec[i];
            }
        }
        let p = &mut ArrayPosition::new();
        let depth = SIZE + 1;
        solve_top(p, depth);
        unsafe {
            self.fitness = NODE_COUNT;
        }
    }
}

const POPULATION_SIZE: usize = 20;
const MUTATION_THRESHOLD: f64 = 0.2;

struct Population {
    vectors: Vec<Vector>,
    generation: usize,
    best: Option<Vector>,
    best_fit: usize,
}

impl Population {
    pub fn new() -> Population {
        let mut vectors = vec![];

        vectors.push(Vector::from_weights([
            25, 87, 63, 98, 80, 212, 5, 1247, 91, 990, 31, 252, 2707, 2695, 357, 1481, 53, 199, 669, -9,
            16, 232, -94, -61, 14, 68, -181, -964, -279, -172, 3, 331, -70, -171, 1403, -562, -12, 352,
            -333, -310, 1069, 1594, 32, -313, -137, -107, -69, -264, 102, 82, 400, 3779, -41, -369, 666,
            262, 537, 526, 4471, -448, 93, 302, 115, 82, 817,
        ]));

        for _ in 0..POPULATION_SIZE-1 {
            vectors.push(Vector::rand());
        }

        Population {
            vectors,
            generation: 0,
            best: None,
            best_fit: 0,
        }
    }

    pub fn evaluate_all(self: &mut Self) {
        for (i, vector) in self.vectors.iter_mut().enumerate() {
            vector.evaluate();
            println!("{:2} ==> {:10}", i, vector.fitness.separate_with_commas());
        }

        self.generation += 1;
    }

    pub fn print(self: &Self) {
        println!("Iteration #{:4}", self.generation);
        for (i, vector) in self.vectors.iter().enumerate() {
            println!("{:2} ==> {:10}", i, vector.fitness.separate_with_commas());
        }
    }

    pub fn shrink(self: &mut Self) {
        self.vectors.sort_by_key(|a| a.fitness);
        if self.best.is_none() || self.vectors[0].fitness < self.best_fit {
            self.best = Some(self.vectors[0].clone());
            self.best_fit = self.vectors[0].fitness;

            println!("---- NEW BEST = {}----", self.best_fit);
            println!("{:?}", self.best.as_ref().unwrap().vec);
        }
        while self.vectors.len() > POPULATION_SIZE {
            self.vectors.pop();
        }
    }

    pub fn crossover(self: &mut Self, num: usize) {
        let mut rng = thread_rng();
        // TODO: replace with a better selection algorithm
        let a = rng.gen_range(0..POPULATION_SIZE);
        let mut b = rng.gen_range(0..POPULATION_SIZE);
        while a == b {
            b = rng.gen_range(0..POPULATION_SIZE);
        }
        let vaf = self.vectors[a].fitness;
        let vbf = self.vectors[b].fitness;

        let (mut v1, mut v2) = if num == 0 {
            self.vectors[a].crossover(&self.vectors[b])
        } else {
            self.vectors[a].crossover2(&self.vectors[b])
        };

        v1.evaluate();
        v2.evaluate();
        self.generation += 1;
        let best = vaf.max(vbf);
        println!("\n== CROSSOVER {} == ", num);
        println!("parent 1 ==> {:10}", vaf.separate_with_commas());
        println!("parent 2 ==> {:10}", vbf.separate_with_commas());
        println!(" ---> ");
        println!(
            "v1 ==> {:10} {}",
            v1.fitness.separate_with_commas(),
            if v1.fitness < best { "SUPER" } else { "" }
        );
        println!(
            "v2 ==> {:10} {}",
            v2.fitness.separate_with_commas(),
            if v2.fitness < best { "SUPER" } else { "" }
        );
        if v1.fitness != vaf && v1.fitness != vbf {
            self.vectors.push(v1);
        }
        if v2.fitness != vaf && v2.fitness != vbf {
            self.vectors.push(v2);
        }
    }

    pub fn mutation(self: &mut Self) {
        let mut rng = thread_rng();
        for vec in self.vectors.iter_mut() {
            if rng.gen_range(0.0..1.0) < MUTATION_THRESHOLD {
                let index = rng.gen_range(0..PATTERN_NUM);
                let fit_before = vec.fitness;
                vec.vec[index] += rng.gen_range(-1000..1000);
                vec.evaluate();
                let fit_after = vec.fitness;
                println!("\nMUTATION: {} ==> {}", fit_before, fit_after);
                if fit_after < fit_before {
                    println!("\t==> IMPROVED");
                }
            }
        }
    }
}

pub fn evolution() {
    let mut population = Population::new();
    population.evaluate_all();
    loop {
        population.shrink();
        population.print();
        population.crossover(0);
        population.crossover(1);
        population.mutation();
    }
}