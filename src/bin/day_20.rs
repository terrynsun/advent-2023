use std::collections::HashMap;

use regex::Regex;

use advent_2023::puzzle::Puzzle;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Freq {
    High,
    Low,
}

impl std::fmt::Display for Freq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Freq::High => write!(f, "-high->"),
            Freq::Low => write!(f, "-low->"),
        }
    }
}

trait Pulse {
    fn pulse(&mut self, kind: Freq, from: String) -> Option<Freq>;

    fn add_input(&mut self, input: String);
}

#[derive(Default)]
struct FlipFlopModule {
    state: bool,
}

impl Pulse for FlipFlopModule {
    // ignored
    fn pulse(&mut self, kind: Freq, _from: String) -> Option<Freq> {
        match kind {
            Freq::High => None,
            Freq::Low => {
                // flips state
                // off -> on: sends a high pulse
                // on -> off: sends a low pulse
                self.state = !self.state;
                if self.state {
                    // high pulse
                    Some(Freq::High)
                } else {
                    // low pulse
                    Some(Freq::Low)
                }
            }
        }
    }

    fn add_input(&mut self, _input: String) {}
}

#[derive(Default)]
struct ConjunctionModule {
    // default to low when not in map
    memory: HashMap<String, Freq>,
}

impl ConjunctionModule {
    fn all_high(&self) -> bool {
        self.memory.iter().all(|(_, &v)| v == Freq::High)
    }
}

impl Pulse for ConjunctionModule {
    fn pulse(&mut self, kind: Freq, from: String) -> Option<Freq> {
        self.memory.insert(from, kind);
        if self.all_high() {
            Some(Freq::Low)
        } else {
            Some(Freq::High)
        }
    }

    fn add_input(&mut self, input: String) {
        self.memory.insert(input, Freq::Low);
    }
}

#[derive(Default)]
struct Broadcaster {}

impl Pulse for Broadcaster {
    fn pulse(&mut self, kind: Freq, _from: String) -> Option<Freq> {
        Some(kind)
    }

    fn add_input(&mut self, _input: String) {}
}

#[derive(Debug, Clone)]
struct Action {
    target: String,
    kind: Freq,
    from: String,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.from, self.kind, self.target)
    }
}

struct Modules {
    modules: HashMap<String, Box<dyn Pulse>>,
    dsts: HashMap<String, Vec<String>>,
}

impl Modules {
    fn from_text(text: Vec<String>) -> Self {
        let mut modules: HashMap<String, Box<dyn Pulse>> = HashMap::new();
        let mut dsts_map = HashMap::new();
        for line in text.iter() {
            let re = Regex::new(r"([&%]?)([a-z]+) -> (.*)").unwrap();
            let (_full, [kind, name, dsts_str]) = re.captures(line).map(|c| c.extract()).unwrap();
            let name = name.to_string();
            let dsts: Vec<_> = dsts_str.split(',').map(|s| s.trim().to_string()).collect();

            match kind {
                "" => modules.insert(name.clone(), Box::<Broadcaster>::default()),
                "%" => modules.insert(name.clone(), Box::<FlipFlopModule>::default()),
                "&" => modules.insert(name.clone(), Box::<ConjunctionModule>::default()),
                _ => panic!("invalid module kind"),
            };

            dsts_map.insert(name, dsts);
        }

        // default ConjuctionModules inputs to low
        for (module_name, module_dsts) in dsts_map.iter_mut() {
            for dst in module_dsts.iter_mut() {
                if let Some(m) = modules .get_mut(dst) {
                    m.add_input(module_name.clone());
                }
            }
        }

        Modules {
            modules,
            dsts: dsts_map,
        }
    }

    fn send_pulse(&mut self, a: Action) -> Vec<Action> {
        //println!("{a}");
        let Some(module) = &mut self.modules.get_mut(&a.target) else { return vec![] };
        if let Some(output_freq) = module.pulse(a.kind, a.from.clone()) {
            self.dsts[&a.target]
                .iter()
                .map(|dst| Action {
                    target: dst.clone(),
                    kind: output_freq,
                    from: a.target.clone(),
                })
                .collect()
        } else {
            vec![]
        }
    }

    // count (low pulses, high pulses)
    fn push_button(&mut self) -> Vec<Action> {
        let mut history = vec![];
        let mut stack = vec![];
        stack.push(Action {
            target: "broadcaster".to_string(),
            kind: Freq::Low,
            from: "button".to_string(),
        });

        loop {
            let pulse = stack.remove(0);
            history.push(pulse.clone());

            let mut result = self.send_pulse(pulse);
            stack.append(&mut result);

            if stack.is_empty() {
                break;
            }
        }

        history
    }
}

fn a(text: &Vec<String>) -> u64 {
    // hack to get around Puzzle limitations...
    // need modules to be mutable, but we can't clone() inthe input to this fn because of trait
    // objects
    let mut modules = Modules::from_text(text.to_vec());

    let (low, high) = (0..1000)
        .flat_map(|_| modules.push_button())
        .fold((0, 0), |acc, x| {
            if x.kind == Freq::Low {
                (acc.0 + 1, acc.1)
            } else {
                (acc.0, acc.1 + 1)
            }
        });

    println!("{low} {high}");
    low * high
}

fn b(text: &Vec<String>) -> u64 {
    // hack to get around Puzzle limitations...
    // need modules to be mutable, but we can't clone() inthe input to this fn because of trait
    // objects
    let mut _modules = Modules::from_text(text.to_vec());

    0
}

fn main() {
    Puzzle {
        name: "20",
        parts: vec![a, b],
        delimiter: '\n',
        preprocess: |text| text,
    }
    .solve();
}
