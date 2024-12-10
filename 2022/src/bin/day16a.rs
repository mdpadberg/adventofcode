use aoc2022::util::read_data_for_day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("{}", solve(read_data_for_day("16").unwrap()));
}

fn solve(input: String) -> u32 {
    let valves: HashMap<ValveName, ValveData> = input
        .split("\n")
        .map(|line| to_valve(line))
        .collect::<HashMap<ValveName, ValveData>>();
    bfs(valves)
}

//TODO: this will not work for 16 deel 2
fn bfs(valves: HashMap<ValveName, ValveData>) -> u32 {
    let all_valves_with_positive_flowrate: Vec<ValveName> = valves
        .iter()
        .filter(|(_, value)| value.flowrate > 0)
        .map(|(key, _)| key.clone())
        .collect();
    let mut visited: HashSet<State> = HashSet::new();
    let mut queue: VecDeque<(Time, State)> = VecDeque::from([(Time(30), State::default())]);
    let mut best: u32 = 0;
    while let Some((time, state)) = queue.pop_front() {
        //time is up
        if time == Time(0) {
            // check if current score is better than the best known score
            if state.totalpressure > best {
                best = state.totalpressure;
            }
            continue;
        }

        if visited.contains(&state) {
            //skip already visited states
            continue;
        } else {
            //insert state in hashset
            visited.insert(state.to_owned());
        }

        let pressure_released_this_round = state
            .openvalves
            .iter()
            .map(|valve| valves.get(valve).unwrap().flowrate)
            .sum::<u32>();

        // skip moving if all valves with positive flowrate are open
        if all_valves_with_positive_flowrate.eq(&state.openvalves) {
            queue.push_back((
                Time(time.0 - 1),
                State {
                    currentvalve: state.currentvalve.clone(),
                    totalpressure: state.totalpressure + pressure_released_this_round,
                    openvalves: state.openvalves.clone(),
                },
            ));
            continue;
        }

        // open current valve if flowrate is more than 0 and it is not open yet
        if !state.openvalves.contains(&state.currentvalve)
            && valves.get(&state.currentvalve).unwrap().flowrate != 0
        {
            let mut openvalves = state.openvalves.clone();
            openvalves.push(state.currentvalve.clone());
            queue.push_back((
                Time(time.0 - 1),
                State {
                    currentvalve: state.currentvalve.clone(),
                    totalpressure: state.totalpressure + pressure_released_this_round,
                    openvalves,
                },
            ));
        }
        // try to go to one of the connecting valves
        else {
            for valve in &valves.get(&state.currentvalve).unwrap().connectingvalves {
                queue.push_back((
                    Time(time.0 - 1),
                    State {
                        currentvalve: ValveName(valve.to_owned()),
                        totalpressure: state.totalpressure + pressure_released_this_round,
                        openvalves: state.openvalves.clone(),
                    },
                ))
            }
        }
    }
    best
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); \w+ \w+ to \w+ (.*)").unwrap();
}

fn to_valve(input: &str) -> (ValveName, ValveData) {
    let captures = REGEX.captures(input).unwrap();
    (
        ValveName(captures.get(1).map(|f| f.as_str().to_string()).unwrap()),
        ValveData {
            flowrate: captures
                .get(2)
                .map(|f| f.as_str().parse::<u32>().unwrap())
                .unwrap(),
            connectingvalves: captures
                .get(3)
                .map(|f| {
                    f.as_str()
                        .split(",")
                        .map(|valve| valve.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap(),
        },
    )
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct ValveName(String);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct ValveData {
    flowrate: u32,
    connectingvalves: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Time(u32);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    totalpressure: u32,
    openvalves: Vec<ValveName>,
    currentvalve: ValveName,
}

impl State {
    fn default() -> State {
        State {
            totalpressure: 0,
            openvalves: vec![],
            currentvalve: ValveName(String::from("AA")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2022::util::read_test_data_for_day;

    #[test]
    fn solvetest() {
        assert_eq!(1650, solve(read_test_data_for_day("16-0").unwrap()));
    }
}
