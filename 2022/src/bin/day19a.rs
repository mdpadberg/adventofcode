use std::collections::{HashSet, VecDeque};
use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("{}", solve(read_data_for_day("19").unwrap()));
}

fn solve(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| Blueprint::from(line))
        .map(|blueprint| bfs(&blueprint) * blueprint.id)
        .sum::<u32>()
}

fn bfs(blueprint: &Blueprint) -> u32 {
    let mut visited: HashSet<State> = HashSet::new();
    let mut queue: VecDeque<(Time, State)> = VecDeque::from([(Time::default(), State::default())]);
    let mut best: u32 = 0;

    while let Some((time, state)) = queue.pop_front() {
        //time is up
        if time == Time(0) {
            // check if current score is better than the best known score
            if state.amount_of_geode > best {
                best = state.amount_of_geode
            }
            continue;
        }

        if visited.contains(&state) {
            //skip already visited states
            continue;
        } else {
            //insert state in hashset
            visited.insert(state);
        }

        //try doing nothing
        queue.push_back((Time(time.0 - 1), state.robot_collecting_materials()));

        //try making ore bot, however dont make a new bot if you already have enough bots (else it will take forever to run)
        if state.amount_of_ore >= blueprint.ore_robot_ore_costs
            && state.amount_of_ore_robots < blueprint.max_ore_costs()
        {
            let mut new_state = state.robot_collecting_materials();
            new_state.amount_of_ore -= blueprint.ore_robot_ore_costs;
            new_state.amount_of_ore_robots += 1;
            queue.push_back((Time(time.0 - 1), new_state));
        }

        //try making clay bot, however dont make a new bot if you already have enough bots (else it will take forever to run)
        if state.amount_of_ore >= blueprint.clay_robot_ore_costs
            && state.amount_of_clay_robots < blueprint.max_clay_costs()
        {
            let mut new_state = state.robot_collecting_materials();
            new_state.amount_of_ore -= blueprint.clay_robot_ore_costs;
            new_state.amount_of_clay_robots += 1;
            queue.push_back((Time(time.0 - 1), new_state));
        }

        //try making obsidian bot, however dont make a new bot if you already have enough bots (else it will take forever to run)
        if state.amount_of_ore >= blueprint.obsidian_robot_ore_costs
            && state.amount_of_clay >= blueprint.obsidian_robot_clay_costs
            && state.amount_of_obsidian_robots < blueprint.max_obsidian_costs()
        {
            let mut new_state = state.robot_collecting_materials();
            new_state.amount_of_ore -= blueprint.obsidian_robot_ore_costs;
            new_state.amount_of_clay -= blueprint.obsidian_robot_clay_costs;
            new_state.amount_of_obsidian_robots += 1;
            queue.push_back((Time(time.0 - 1), new_state));
        }

        //try making geode bot
        if state.amount_of_ore >= blueprint.geode_robot_ore_costs
            && state.amount_of_obsidian >= blueprint.geode_robot_obsidian_costs
        {
            let mut new_state = state.robot_collecting_materials();
            new_state.amount_of_ore -= blueprint.geode_robot_ore_costs;
            new_state.amount_of_obsidian -= blueprint.geode_robot_obsidian_costs;
            new_state.amount_of_geode_robots += 1;
            queue.push_back((Time(time.0 - 1), new_state));
        }
    }

    return best;
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_costs: u32,
    clay_robot_ore_costs: u32,
    obsidian_robot_ore_costs: u32,
    obsidian_robot_clay_costs: u32,
    geode_robot_ore_costs: u32,
    geode_robot_obsidian_costs: u32,
}

impl Blueprint {
    fn max_ore_costs(&self) -> u32 {
        vec![
            self.ore_robot_ore_costs,
            self.clay_robot_ore_costs,
            self.obsidian_robot_ore_costs,
            self.geode_robot_ore_costs,
        ]
        .iter()
        .max()
        .unwrap()
        .to_owned()
    }

    fn max_clay_costs(&self) -> u32 {
        vec![self.obsidian_robot_clay_costs]
            .iter()
            .max()
            .unwrap()
            .to_owned()
    }

    fn max_obsidian_costs(&self) -> u32 {
        vec![self.geode_robot_obsidian_costs]
            .iter()
            .max()
            .unwrap()
            .to_owned()
    }
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

impl Blueprint {
    fn from(str: &str) -> Blueprint {
        REGEX
            .captures(str)
            .map(|values| Blueprint {
                id: values[1].parse::<u32>().unwrap(),
                ore_robot_ore_costs: values[2].parse::<u32>().unwrap(),
                clay_robot_ore_costs: values[3].parse::<u32>().unwrap(),
                obsidian_robot_ore_costs: values[4].parse::<u32>().unwrap(),
                obsidian_robot_clay_costs: values[5].parse::<u32>().unwrap(),
                geode_robot_ore_costs: values[6].parse::<u32>().unwrap(),
                geode_robot_obsidian_costs: values[7].parse::<u32>().unwrap(),
            })
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Time(u32);

impl Default for Time {
    fn default() -> Self {
        Self(24)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    amount_of_ore: u32,
    amount_of_ore_robots: u32,
    amount_of_clay: u32,
    amount_of_clay_robots: u32,
    amount_of_obsidian: u32,
    amount_of_obsidian_robots: u32,
    amount_of_geode: u32,
    amount_of_geode_robots: u32,
}

impl State {
    fn robot_collecting_materials(self) -> State {
        State {
            amount_of_ore: self.amount_of_ore + self.amount_of_ore_robots,
            amount_of_clay: self.amount_of_clay + self.amount_of_clay_robots,
            amount_of_obsidian: self.amount_of_obsidian + self.amount_of_obsidian_robots,
            amount_of_geode: self.amount_of_geode + self.amount_of_geode_robots,
            ..self
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            amount_of_ore: 0,
            amount_of_ore_robots: 1,
            amount_of_clay: 0,
            amount_of_clay_robots: 0,
            amount_of_obsidian: 0,
            amount_of_obsidian_robots: 0,
            amount_of_geode: 0,
            amount_of_geode_robots: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;
    #[test]
    fn solvetest() {
        assert_eq!(33, solve(read_test_data_for_day("19-0").unwrap()));
    }
}
