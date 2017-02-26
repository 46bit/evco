extern crate rand;
extern crate evco;

use std::fmt;
use rand::{OsRng, Rng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use evco::gp::*;
use evco::gp::tree::*;

// ----------------------------------------------------------------------------------------------

// CUSTOM DERIVE OPTION A

#[derive(Clone, Debug, PartialEq, Eq, Tree)]
enum Equation {
    Add(Node<Equation>, Node<Equation>),
    Sub(Node<Equation>, Node<Equation>),
    Mul(Node<Equation>, Node<Equation>),
    Div(Node<Equation>, Node<Equation>),
    Neg(Node<Equation>),
    Sin(Node<Equation>),
    Cos(Node<Equation>),
    Int(EquationInt),
    Input,
}

struct EquationInt(i64);

impl Rand for EquationInt {
    fn gen(r) -> EquationInt {
        EquationInt(r.gen_range(-1, 2))
    }
}

// The `Tree` custom derivation will identify terminals because they aren't `Box<Self>`. They shall
// simply be generated using `Rng::gen`. To use custom generation logic you use a Newtype and take
// out the contents in your `eval` implementation.

// ----------------------------------------------------------------------------------------------

// CUSTOM DERIVE OPTION D

// Reduce the weight of newtypes with simple attributes.
// * Add a `gen` attribute specifying a `Path` to an `FnMut(R) -> i64 where R: Rng`.
// * Add a `range` attribute specifying arguments for `Rng::gen_range`.
// ? Add a `rng` attribute specifying how to use `rng`: `#[rng = "gen_range(-1, 2)"]`.
//   I don't using this should *ever* be expected or encouraged.

#[derive(Clone, Debug, PartialEq, Eq, Tree)]
enum Equation {
    Add(Node<Equation>, Node<Equation>),
    Sub(Node<Equation>, Node<Equation>),
    Mul(Node<Equation>, Node<Equation>),
    Div(Node<Equation>, Node<Equation>),
    Neg(Node<Equation>),
    Sin(Node<Equation>),
    Cos(Node<Equation>),
    Int(#[range = "-1..2"] i64),
    Float(#[gen = "gen_f64"] f64),
    Input,
}

// ----------------------------------------------------------------------------------------------

// For several reasons we want to split evaluation into a separate trait. Evolutionary agents in
// general should be evaluable with the below. This is also necessary in order to autoderive the
// `Tree` trait. The `Individual` type should probably be parametric over an `Evaluable` genotype
// rather than `Tree`. It could then be moved out of the `gp` module, and trees generated with:
//
//     let indv: Individual<Equation> = Individual::new(&mut tree_gen);
//
// That is, `TreeGen<T>` will implement a new `GenotypeBuilder` trait with a generator method.
// This can't just be `Rand` as that wouldn't allow a for producing anything but another
// `GenotypeBuilder`. Anyhow, this implementation will use `T::new_tree(&mut self)`.
//
// I don't think `GenotypeBuilder`s should be required to be `Rng` but in general it always
// will be one.
//
// Oh. `TreeGen` should also be renamed. How about `gp::TreeBuilder`?
//
// `BoxTree` has already been renamed above to `Node`. This obscures what it does slightly but
// will look much more natural. `futures::BoxFuture` is a nice touch (albeit under debate there
// for implementation reasons) but `evco::gp::tree::BoxTree` is too obscure for even to follow.
//
// So it's now `gp::tree::{Tree, Node, TreeBuilder}`. Merging those into `gp` starts to look a
// bit more tempted - it just needs usage examples in the documentation.

impl Evaluable {
    type Input = f64;
    type Output = f64;

    fn eval(&self, env: &Self::Input) -> Self::Output {
        use Equation::*;
        match *self {
            Add(ref left, ref right) => left.eval(env) + right.eval(env),
            Sub(ref left, ref right) => left.eval(env) - right.eval(env),
            Mul(ref left, ref right) => left.eval(env) * right.eval(env),
            Div(ref left, ref right) => protected_div(left.eval(env), right.eval(env)),
            Neg(ref left) => -left.eval(env),
            Sin(ref left) => left.eval(env).sin(),
            Cos(ref left) => left.eval(env).cos(),
            Int(i) => i as f64,
            Input => *env,
        }
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Equation::*;
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
