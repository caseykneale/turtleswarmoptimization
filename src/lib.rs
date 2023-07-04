use std::f64::{EPSILON, INFINITY};

use ndarray::{Array, Array1};
use ndarray_rand::{rand_distr::Uniform, RandomExt};

// Turtles move slowly and methodically, whatever you do don't change this parameter.
// Especially do not raise it's value, or you will lose all of the advantages of the
// turtle swarm optimizer (TSO).
const TURTLE_VELOCITY: f64 = EPSILON;

#[derive(Clone, Copy)]
pub struct CubicBoundary {
    pub lower: f64,
    pub upper: f64,
    pub shape: usize,
}

impl CubicBoundary {
    pub fn new(shape: usize, lower: f64, upper: f64) -> Self {
        // Just incase someone makes a mistake here.
        if lower > upper {
            Self {
                lower: upper,
                upper: lower,
                shape,
            }
        } else {
            Self {
                lower,
                upper,
                shape,
            }
        }
    }
}

// Turtles are similar to `particles` in particle swarm optimization strategies.
// They store a location, velocity, and local best scoring information.
#[derive(Debug)]
pub struct Turtle {
    pub position: Array1<f64>,
    velocity: Array1<f64>,
    pub best_score: f64,
    pub best_position: Array1<f64>,
}

impl Turtle {
    pub fn new(boundaries: &CubicBoundary) -> Self {
        Self {
            position: Array::random(
                boundaries.shape,
                Uniform::new(boundaries.lower, boundaries.upper),
            ),
            velocity: EPSILON * Array1::ones(boundaries.shape),
            best_score: INFINITY,
            best_position: Array1::zeros(boundaries.shape),
        }
    }
}

/// The Optimizer type is the core of this library. It defines how the optimization/minimization
/// process should proceed. One important deviation between the TSO and PSO algorithms is, the TSO
/// offers no early exit from achieving your goal (a minimum acceptabe float point value).
/// This rule was derived to follow biomimetic inspiration.
pub struct Optimizer<'a> {
    pub turtles: Vec<Turtle>,
    pub boundaries: CubicBoundary,
    pub iterations: usize,
    pub best_score: f64,
    pub best_position: Array1<f64>,
    pub objective_function: &'a dyn Fn(&Array1<f64>) -> f64,
    pub goal: f64,
}

impl<'a> Optimizer<'a> {
    pub fn new(
        turtles: usize,
        boundaries: CubicBoundary,
        objective_function: &'a dyn Fn(&Array1<f64>) -> f64,
        goal: f64,
    ) -> Self {
        Self {
            turtles: (0..turtles)
                .map(|_| Turtle::new(&boundaries))
                .collect::<Vec<Turtle>>(),
            boundaries,
            iterations: 0,
            best_score: INFINITY,
            best_position: Array1::zeros(boundaries.shape),
            objective_function,
            goal,
        }
    }

    fn evaluate(&mut self) {
        for turtle in self.turtles.iter_mut() {
            let score = (self.objective_function)(&turtle.position);
            if score < turtle.best_score {
                turtle.best_score = score;
                turtle.best_position = turtle.position.clone();
                if score < self.best_score {
                    self.best_score = score;
                    self.best_position = turtle.position.clone();
                }
            }
        }
    }

    fn update_velocities(&mut self) {
        for turtle in self.turtles.iter_mut() {
            // Here we deviate from Kennedy and Eberhart and omit social and personal motivation constants.
            // We aren't sure exactly what motivates turtles so we favor neither in a stochastic sense.
            turtle.velocity = &turtle.velocity
                + TURTLE_VELOCITY * (&turtle.best_position - &turtle.position)
                + TURTLE_VELOCITY * (&self.best_position - &turtle.position);
        }
    }

    fn update_positions(&mut self) {
        for turtle in self.turtles.iter_mut() {
            turtle.position = &turtle.position + &turtle.velocity;
            for dimension in turtle.position.iter_mut() {
                if *dimension > self.boundaries.upper {
                    *dimension = self.boundaries.upper;
                } else if *dimension < self.boundaries.lower {
                    *dimension = self.boundaries.lower;
                }
            }
        }
    }

    /// The optimize method iterates the TSO algorithm until the best observed score matches what the user
    /// set as their goal.  
    pub fn optimize(&mut self) {
        // Instead of allowing the user of this library to terminate early based on the number of iterations
        // we put the turtles to work until they reach our goal. No animals were harmed in the making of this.
        while self.best_score > self.goal {
            self.evaluate();
            self.update_velocities();
            self.update_positions();
            self.iterations += 1;
        }
    }

    /// Reports the results of a completed optimization to stdout.
    pub fn report(&self) {
        println!(
            "{} turtles performed {} optimizer iterations for you.",
            self.turtles.len(),
            self.iterations
        );
        println!(
            "The best score: {} was observed at position: {}",
            self.best_score, self.best_position
        );
        println!("Below is a complete run down of the best locations: ");

        for (turtle_number, turtle) in self.turtles.iter().enumerate() {
            println!(
                "\t Turtle #{}'s best score {}, was observed at {} ",
                turtle_number, turtle.best_score, turtle.best_position
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sufficient_testing() {
        fn parabola(x: &Array1<f64>) -> f64 {
            (x * x).sum()
        }
        let boundaries = CubicBoundary::new(1, -1., 1.);
        let mut optimizer = Optimizer::new(33, boundaries, &parabola, 1e-2);

        optimizer.evaluate();

        for turtle in optimizer.turtles.iter() {
            assert_ne!(turtle.best_score, INFINITY);
        }

        optimizer.update_velocities();

        let movement = optimizer
            .turtles
            .iter()
            .map(|turtle| turtle.velocity.sum())
            .any(|v_sum| v_sum != 0.0);
        assert!(movement);

        optimizer.optimize();
        optimizer.report();
    }
}
