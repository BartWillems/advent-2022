enum Input {
    Calories(usize),
    Separator,
}

struct State {
    /// List of elves with their current callorie count
    elves: Vec<usize>,
    /// Currently selected elf
    current_elf: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            elves: vec![0],
            current_elf: 0,
        }
    }
}

impl State {
    fn new_elf(&mut self) {
        self.elves.push(0);
        self.current_elf += 1;
    }

    fn add_input(&mut self, input: Input) {
        match input {
            Input::Calories(count) => self.elves[self.current_elf] += count,
            Input::Separator => self.new_elf(),
        }
    }

    /// Get the max amount of calories an elf is carrying
    fn max_calories(&self) -> usize {
        *self.elves.iter().max().unwrap()
    }

    fn top_three_max_calories(&self) -> usize {
        let mut calories = self.elves.clone();
        calories.sort();
        calories.reverse();

        calories.iter().take(3).sum()
    }
}

fn main() {
    let lines = utils::load_file("inputs/day-1.txt");

    let mut state = State::default();

    lines
        .iter()
        .map(|line| match line.as_str() {
            "" => Input::Separator,
            calories => Input::Calories(calories.parse().unwrap()),
        })
        .for_each(|input| state.add_input(input));

    println!("Max amount of calories: {}", state.max_calories());
    println!("Top 3 max calories: {}", state.top_three_max_calories());
}
