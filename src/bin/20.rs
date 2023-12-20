advent_of_code::solution!(20);

use std::collections::{HashMap, HashSet, VecDeque};

// % == flip-flop.  Off to start, changes when low pulse.  Off > on == high pulse.  On > off == low
//   pulse.
// & == conjunction.  Initially low pulse in each input.  Remembers values to inputs.  After
//   update, if all inputs are high pulse == send low pulse.  Else == send high pulse
// broadcaster == broadcast input to all outputs
// button == low pulse is sent to broadcast module

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn from_flip(flip_state: bool) -> Self {
        match flip_state {
            false => Self::Low,
            true => Self::High,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Module {
    FlipFlop(bool, Vec<String>),                    // state, outputs
    Conjunction(Vec<(String, Pulse)>, Vec<String>), // inputs, outputs
}

impl Module {
    fn with_name_from(line: &str) -> (String, Self) {
        let parts: Vec<_> = line.split(" -> ").collect();
        if line.starts_with("%") {
            let name = &parts[0][1..];
            let outputs = parts[1].split(", ").map(|s| s.to_owned()).collect();
            (name.to_owned(), Self::FlipFlop(false, outputs))
        } else if line.starts_with("&") {
            let name = &parts[0][1..];
            let outputs = parts[1].split(", ").map(|s| s.to_owned()).collect();
            (name.to_owned(), Self::Conjunction(Vec::new(), outputs))
        } else {
            panic!("Unexpected line: {}", line);
        }
    }

    fn conj_state(&self) -> Option<Vec<Pulse>> {
        match self {
            Self::FlipFlop(_, _) => None,
            Self::Conjunction(state, _) => Some(state.iter().map(|item| item.1).collect()),
        }
    }

    fn update_state(&mut self, names: Vec<String>) -> () {
        if let Self::Conjunction(ref mut state, _) = self {
            *state = names.iter().map(|s| (s.to_owned(), Pulse::Low)).collect();
        }
    }

    fn handle_input(&mut self, input: Pulse, input_name: &str) -> Option<(Pulse, Vec<String>)> {
        // result, outputs
        match self {
            Self::FlipFlop(ref mut state, outputs) => match input {
                Pulse::High => None,
                Pulse::Low => {
                    *state = !*state;
                    Some((Pulse::from_flip(*state), outputs.to_vec()))
                }
            },
            Self::Conjunction(ref mut state, outputs) => {
                state.iter_mut().for_each(|item| {
                    if item.0 == input_name {
                        *item = (input_name.to_owned(), input);
                    }
                });
                if state.iter().all(|(_, p)| matches!(p, Pulse::High)) {
                    Some((Pulse::Low, outputs.to_vec()))
                } else {
                    Some((Pulse::High, outputs.to_vec()))
                }
            }
        }
    }
}

fn press_button(broadcast: &Vec<String>, modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut queue = VecDeque::new();
    let mut hpulses = 0;
    let mut lpulses = 1;
    let add_to_queue = |pulse: Pulse,
                        from: String,
                        items: &Vec<String>,
                        queue: &mut VecDeque<(Pulse, String, String)>,
                        hpulses: &mut usize,
                        lpulses: &mut usize| {
        items.iter().for_each(|s| {
            queue.push_back((pulse, from.to_owned(), s.to_string()));
            match pulse {
                Pulse::High => *hpulses += 1,
                Pulse::Low => *lpulses += 1,
            }
        });
    };
    add_to_queue(
        Pulse::Low,
        "broadcast".to_string(),
        broadcast,
        &mut queue,
        &mut hpulses,
        &mut lpulses,
    );
    while !queue.is_empty() {
        let (pulse, from, name) = queue.pop_front().unwrap();
        if let Some(module) = modules.get_mut(&name) {
            if let Some((new_pulse, outputs)) = module.handle_input(pulse, &from) {
                add_to_queue(
                    new_pulse,
                    name.clone(),
                    &outputs,
                    &mut queue,
                    &mut hpulses,
                    &mut lpulses,
                );
            }
        }
    }
    (hpulses, lpulses)
}

fn press_button_part2(
    broadcast: &Vec<String>,
    modules: &mut HashMap<String, Module>,
    curr_count: usize,
    counts: Vec<usize>,
) -> Vec<usize> {
    let mut counts = counts;
    let mut queue = VecDeque::new();
    let add_to_queue = |pulse: Pulse,
                        from: String,
                        items: &Vec<String>,
                        queue: &mut VecDeque<(Pulse, String, String)>| {
        items.iter().for_each(|s| {
            queue.push_back((pulse, from.to_owned(), s.to_string()));
        });
    };
    add_to_queue(Pulse::Low, "broadcast".to_string(), broadcast, &mut queue);
    while !queue.is_empty() {
        let (pulse, from, name) = queue.pop_front().unwrap();
        if let Some(module) = modules.get_mut(&name) {
            if let Some((new_pulse, outputs)) = module.handle_input(pulse, &from) {
                if outputs.contains(&"rx".to_string()) {
                    let state = module.conj_state().unwrap();
                    state.iter().enumerate().for_each(|(index, p)| {
                        if matches!(p, Pulse::High) {
                            counts[index] = curr_count;
                        }
                    })
                }
                add_to_queue(new_pulse, name.clone(), &outputs, &mut queue);
            }
        }
    }
    counts
}

fn modules_from_input(input: &str) -> (Vec<String>, HashMap<String, Module>, Option<String>) {
    let (broadcast, mut modules, inputs): (
        Vec<String>,
        HashMap<String, Module>,
        HashMap<String, Vec<String>>,
    ) = input.lines().fold(
        (Vec::new(), HashMap::new(), HashMap::new()),
        |mut acc, line| {
            if line.starts_with("broadcaster") {
                let broadcast = line
                    .split(" -> ")
                    .last()
                    .unwrap()
                    .split(", ")
                    .map(|s| s.to_owned())
                    .collect();
                acc.0 = broadcast
            } else {
                let (name, module) = Module::with_name_from(line);
                let outputs = match &module {
                    Module::FlipFlop(_, outputs) => outputs.clone(),
                    Module::Conjunction(_, outputs) => outputs.clone(),
                };
                for output in outputs.iter() {
                    acc.2
                        .entry(output.to_string())
                        .and_modify(|v| v.push(name.clone()))
                        .or_insert(vec![name.clone()]);
                }
                acc.1.insert(name, module);
            }
            acc
        },
    );
    let rx_input = inputs.get("rx").cloned();
    inputs.into_iter().for_each(|(name, inputs)| {
        if let Some(m) = modules.get_mut(&name) {
            m.update_state(inputs);
        }
    });
    if let Some(v) = rx_input {
        if v.len() == 1 {
            return (broadcast, modules, Some(v[0].clone()));
        }
    }
    (broadcast, modules, None)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (broadcast, mut modules, _) = modules_from_input(input);

    let mut seen: HashSet<Vec<Module>> = HashSet::new();
    let mut pulses = Vec::new();
    while !seen.contains(&modules.values().cloned().collect::<Vec<Module>>()) && pulses.len() < 1000
    {
        seen.insert(modules.values().cloned().collect::<Vec<Module>>());
        pulses.push(press_button(&broadcast, &mut modules));
    }

    let res = pulses
        .iter()
        .cycle()
        .take(1000)
        .fold((0, 0), |acc, item| (acc.0 + item.0, acc.1 + item.1));
    Some((res.0 * res.1) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (broadcast, mut modules, rx_input) = modules_from_input(input);
    let rx_input_num_inputs = modules
        .get(&rx_input.unwrap())
        .unwrap()
        .conj_state()
        .expect("Input to rx must be a Conjunction")
        .len();

    let mut count = 0;
    let mut counts = vec![0; rx_input_num_inputs];
    loop {
        count += 1;
        counts = press_button_part2(&broadcast, &mut modules, count, counts);
        if !counts.contains(&0) {
            break;
        }
    }
    Some(lcm(&counts) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_flop_state() {
        let mut flip = Module::FlipFlop(false, Vec::new());
        let _ = flip.handle_input(Pulse::Low, "a");
        if let Module::FlipFlop(state, _) = flip {
            assert_eq!(state, true);
        }
    }

    #[test]
    fn test_conjunction_state() {
        let mut flip = Module::Conjunction(
            vec![("a".to_string(), Pulse::Low), ("b".to_string(), Pulse::Low)],
            Vec::new(),
        );
        let _ = flip.handle_input(Pulse::High, "a");
        if let Module::Conjunction(state, _) = flip {
            assert_eq!(
                state,
                vec![
                    ("a".to_string(), Pulse::High),
                    ("b".to_string(), Pulse::Low)
                ]
            );
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    // example doesn't work for part 2
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
