extern crate rand;
extern crate jeepers;

use std::fmt;
use rand::{OsRng, Rng, Rand};
use std::collections::VecDeque;

use jeepers::gp::{Individual, BoxTree, Tree, TreeGen};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TurnDirection {
    Left,
    Ahead,
    Right,
}

impl Rand for TurnDirection {
    fn rand<R: Rng>(r: &mut R) -> TurnDirection {
        match r.next_u32() % 3 {
            0 => TurnDirection::Left,
            1 => TurnDirection::Ahead,
            2 => TurnDirection::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CompassDirection {
    North,
    East,
    South,
    West,
}

impl CompassDirection {
    fn variants() -> &'static [CompassDirection] {
        static VARIANTS: &'static [CompassDirection] = &[CompassDirection::North,
                                                         CompassDirection::East,
                                                         CompassDirection::South,
                                                         CompassDirection::West];
        VARIANTS
    }
}

impl Rand for CompassDirection {
    fn rand<R: Rng>(r: &mut R) -> CompassDirection {
        match r.next_u32() % 4 {
            0 => CompassDirection::North,
            1 => CompassDirection::East,
            2 => CompassDirection::South,
            3 => CompassDirection::West,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn neighbour(&self, direction: &CompassDirection) -> Vector {
        match *direction {
            CompassDirection::North => {
                Vector {
                    x: self.x,
                    y: self.y - 1,
                }
            }
            CompassDirection::East => {
                Vector {
                    x: self.x + 1,
                    y: self.y,
                }
            }
            CompassDirection::South => {
                Vector {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            CompassDirection::West => {
                Vector {
                    x: self.x - 1,
                    y: self.y,
                }
            }
        }
    }

    fn direction_to(&self, possible_neighbour: Vector) -> Option<CompassDirection> {
        for direction in CompassDirection::variants() {
            let neighbour = self.neighbour(direction);
            if neighbour == possible_neighbour {
                return Some(*direction);
            }
        }
        None
    }
}

pub struct SnakeEnvironment {
    pub size: Vector,
    pub food: Vector,
    pub snake: Vec<Vector>,
}

impl SnakeEnvironment {
    fn turn_to_compass_direction(&self, turn_direction: TurnDirection) -> CompassDirection {
        let snake_current_compass_direction = self.snake[1].direction_to(self.snake[0]).unwrap();
        let directions = CompassDirection::variants();
        let index = directions.iter().position(|&r| r == snake_current_compass_direction).unwrap();
        let new_compass_direction_index = match turn_direction {
            TurnDirection::Left => (index - 1) % 4,
            TurnDirection::Ahead => index,
            TurnDirection::Right => (index + 1) % 4,
        };
        directions[new_compass_direction_index]
    }

    fn sense_danger(&self, turn_direction: TurnDirection) -> bool {
        let compass_direction = self.turn_to_compass_direction(turn_direction);
        let cell_in_direction = self.snake[0].neighbour(&compass_direction);
        (cell_in_direction.x < 0 || cell_in_direction.y < 0 ||
         cell_in_direction.x >= self.size.x || cell_in_direction.y >= self.size.y ||
         self.snake.contains(&cell_in_direction))
    }

    fn sense_food(&self, turn_direction: TurnDirection) -> bool {
        let compass_direction = self.turn_to_compass_direction(turn_direction);
        let cell_in_direction = self.snake[0].neighbour(&compass_direction);
        self.food == cell_in_direction
    }
}

#[derive(Clone, Debug)]
pub enum SnakeTree {
    IfDanger(TurnDirection, BoxTree<SnakeTree>, BoxTree<SnakeTree>),
    IfFood(TurnDirection, BoxTree<SnakeTree>, BoxTree<SnakeTree>),
    Move(TurnDirection),
}

use SnakeTree::*;

impl Tree for SnakeTree {
    type Environment = SnakeEnvironment;
    type Action = TurnDirection;

    fn branch<R: Rng>(tg: &mut TreeGen<R>, current_depth: usize) -> BoxTree<Self> {
        let direction = TurnDirection::rand(tg);
        let true_ = Self::child(tg, current_depth + 1);
        let false_ = Self::child(tg, current_depth + 1);
        if tg.gen() {
            IfDanger(direction, true_, false_).into()
        } else {
            IfFood(direction, true_, false_).into()
        }
    }

    fn leaf<R: Rng>(tg: &mut TreeGen<R>, _: usize) -> BoxTree<Self> {
        Move(TurnDirection::rand(tg)).into()
    }

    fn count_children(&mut self) -> usize {
        match *self {
            IfDanger(_, _, _) |
            IfFood(_, _, _) => 2,
            Move(_) => 0,
        }
    }

    fn children(&self) -> Vec<&BoxTree<Self>> {
        match *self {
            IfDanger(_, ref left_, ref right_) |
            IfFood(_, ref left_, ref right_) => vec![left_, right_],
            Move(_) => vec![],
        }
    }

    fn children_mut(&mut self) -> Vec<&mut BoxTree<Self>> {
        match *self {
            IfDanger(_, ref mut left_, ref mut right_) |
            IfFood(_, ref mut left_, ref mut right_) => vec![left_, right_],
            Move(_) => vec![],
        }
    }

    fn evaluate(&self, env: SnakeEnvironment) -> Self::Action {
        match *self {
            IfDanger(direction, ref left_, ref right_) => {
                if env.sense_danger(direction) {
                    left_.evaluate(env)
                } else {
                    right_.evaluate(env)
                }
            }
            IfFood(direction, ref left_, ref right_) => {
                if env.sense_food(direction) {
                    left_.evaluate(env)
                } else {
                    right_.evaluate(env)
                }
            }
            Move(direction) => direction,
        }
    }
}

impl fmt::Display for SnakeTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stack: VecDeque<&Self> = VecDeque::new();
        let mut depth_stack: VecDeque<usize> = VecDeque::new();
        stack.push_back(self);
        depth_stack.push_back(0);
        while let (Some(node), Some(depth)) = (stack.pop_back(), depth_stack.pop_back()) {
            for _ in 0..depth {
                write!(f, "    ")?;
            }
            match *node {
                IfDanger(direction, _, _) => write!(f, "IfDanger({:?})", direction)?,
                IfFood(direction, _, _) => write!(f, "IfFood({:?})", direction)?,
                Move(direction) => write!(f, "Move({:?})", direction)?,
            }
            write!(f, "\n")?;

            let mut children = node.children();
            children.reverse();
            for child in children {
                stack.push_back(child);
                depth_stack.push_back(depth + 1);
            }
        }
        Ok(())
    }
}

fn main() {
    let mut rng = OsRng::new().unwrap();

    let mut tree_gen = TreeGen::full(&mut rng, 1, 3);
    let p = SnakeTree::tree(&mut tree_gen);
    println!("{}", p);

    let i = Individual::new(p);
    println!("{}", i);
}
