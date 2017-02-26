extern crate rand;
extern crate evco;

use std::fmt;
use rand::{OsRng, Rng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use evco::gp::*;
use evco::gp::tree::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Equation {
    Add(BoxTree<Equation>, BoxTree<Equation>),
    Sub(BoxTree<Equation>, BoxTree<Equation>),
    Mul(BoxTree<Equation>, BoxTree<Equation>),
    Div(BoxTree<Equation>, BoxTree<Equation>),
    Neg(BoxTree<Equation>),
    Sin(BoxTree<Equation>),
    Cos(BoxTree<Equation>),
    Int(i64),
    Input,
}

use Equation::*;

impl Tree for Equation {
    type Environment = f64;
    type Action = f64;

    fn branch<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> BoxTree<Self> {
        let left = Self::child(tg, current_depth + 1);
        let right = Self::child(tg, current_depth + 1);
        match tg.gen_range(0, 7) {
                0 => Add(left, right),
                1 => Sub(left, right),
                2 => Mul(left, right),
                3 => Div(left, right),
                4 => Neg(left),
                5 => Sin(left),
                6 => Cos(left),
                _ => unreachable!(),
            }
            .into()
    }

    fn leaf<R: Rng>(tg: &mut TreeGen<R>, _: usize) -> BoxTree<Self> {
        match tg.gen_range(0, 2) {
                0 => Int(tg.gen_range(-1, 2)),
                1 => Input,
                _ => unreachable!(),
            }
            .into()
    }

    fn count_children(&mut self) -> usize {
        match *self {
            Int(_) => 0,
            _ => 2,
        }
    }

    fn children(&self) -> Vec<&BoxTree<Self>> {
        match *self {
            Add(ref left, ref right) |
            Sub(ref left, ref right) |
            Mul(ref left, ref right) |
            Div(ref left, ref right) => vec![left, right],
            Neg(ref left) | Sin(ref left) | Cos(ref left) => vec![left],
            Int(_) | Input => vec![],
        }
    }

    fn children_mut(&mut self) -> Vec<&mut BoxTree<Self>> {
        match *self {
            Add(ref mut left, ref mut right) |
            Sub(ref mut left, ref mut right) |
            Mul(ref mut left, ref mut right) |
            Div(ref mut left, ref mut right) => vec![left, right],
            Neg(ref mut left) |
            Sin(ref mut left) |
            Cos(ref mut left) => vec![left],
            Int(_) | Input => vec![],
        }
    }

    fn evaluate(&self, env: &Self::Environment) -> Self::Action {
        match *self {
            Add(ref left, ref right) => left.evaluate(env) + right.evaluate(env),
            Sub(ref left, ref right) => left.evaluate(env) - right.evaluate(env),
            Mul(ref left, ref right) => left.evaluate(env) * right.evaluate(env),
            Div(ref left, ref right) => protected_div(left.evaluate(env), right.evaluate(env)),
            Neg(ref left) => -left.evaluate(env),
            Sin(ref left) => left.evaluate(env).sin(),
            Cos(ref left) => left.evaluate(env).cos(),
            Int(i) => i as f64,
            Input => *env,
        }
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Add(ref left, ref right) => write!(f, "{} + {}", left, right),
            Sub(ref left, ref right) => write!(f, "{} - {}", left, right),
            Mul(ref left, ref right) => write!(f, "({}) * ({})", left, right),
            Div(ref left, ref right) => write!(f, "({}) / ({})", left, right),
            Neg(ref left) => write!(f, "-{}", left),
            Sin(ref left) => write!(f, "sin({})", left),
            Cos(ref left) => write!(f, "cos({})", left),
            Int(i) => write!(f, "({})", i),
            Input => write!(f, "x"),
        }
    }
}
// Add(Int(1), Div(Input, Input))
// 1 + x/x
// (1) + (x) / (x)xx(1)(x) / (x)xx
// 1 + x /

fn protected_div(numerator: f64, denominator: f64) -> f64 {
    let div = numerator / denominator;
    if div.is_finite() { div } else { 1.0 }
}

#[derive(Debug, Clone)]
struct RankedIndividual(f64, Individual<Equation>);

impl Ord for RankedIndividual {
    fn cmp(&self, other: &RankedIndividual) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Greater)
    }
}

impl PartialOrd for RankedIndividual {
    fn partial_cmp(&self, other: &RankedIndividual) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RankedIndividual {
    fn eq(&self, other: &RankedIndividual) -> bool {
        self.0 == other.0
    }
}

impl Eq for RankedIndividual {}

fn main() {
    let mut rng = OsRng::new().unwrap();
    let mut tree_gen = TreeGen::full(&mut rng, 1, 4);

    let mut rng = OsRng::new().unwrap();
    let crossover = Crossover::one_point();

    let mut mutate_rng = OsRng::new().unwrap();
    let mut mut_tree_gen = TreeGen::full(&mut mutate_rng, 1, 2);
    let mutation = Mutation::uniform();

    let inputs: Vec<f64> = (-10..11).map(|i| (i as f64) / 10.0).collect();
    let expecteds: Vec<f64> = inputs.iter()
        .cloned()
        .map(|i| i.powi(4) + i.powi(3) + i.powi(2) + i)
        .collect();

    let mut population: Vec<Individual<Equation>> =
        (0..200).map(|_| Individual::new(&mut tree_gen)).collect();
    for round in 0..40 {
        let mut ranking = BinaryHeap::new();
        for individual in population.drain(..) {
            let mut sum_of_squared_errors = 0.0;
            for i in 0..inputs.len() {
                let input = inputs[i];
                let expected = expecteds[i];
                let output = individual.tree.evaluate(&input);
                let squared_error = (output - expected).powi(2);
                sum_of_squared_errors += squared_error;
            }
            if !sum_of_squared_errors.is_finite() {
                sum_of_squared_errors = 100000000000.0;
            }
            ranking.push(RankedIndividual(sum_of_squared_errors, individual));
        }

        let ranking = ranking.into_sorted_vec();
        //println!("{:?}", ranking);

        println!("=== ROUND {} ===", round);
        for i in 0..3 {
            println!("Rank {:?}\n  Range = [-1.0, 1.0]    Step = +0.1\n  Comparing to x^4 + x^3 \
                      + x^2 + x\n  Sum of squared error = {}\n  Equation = {}",
                     i,
                     ranking[i].0,
                     ranking[i].1);
        }

        for i in 0..100 {
            let RankedIndividual(_, mut indv1) = ranking[i].clone();
            let RankedIndividual(_, mut indv2) = ranking[i + 1].clone();

            population.push(indv1.clone());
            population.push(indv2.clone());

            crossover.mate(&mut indv1, &mut indv2, &mut rng);

            if rng.gen() {
                mutation.mutate(&mut indv1, &mut mut_tree_gen);
            }
            if rng.gen() {
                mutation.mutate(&mut indv2, &mut mut_tree_gen);
            }

            population.push(indv1);
            population.push(indv2);
        }

        println!();
    }
}
