//! <https://adventofcode.com/2021/day/6>

const INPUT: [u32; 300] = [
    5, 1, 1, 4, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 2, 1, 1, 1, 3, 5, 1, 1, 1, 5, 4, 1, 1,
    1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 5, 2, 1, 2, 2, 3, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 4, 1, 1, 1,
    5, 4, 1, 1, 3, 3, 2, 1, 1, 1, 5, 1, 1, 4, 1, 1, 5, 1, 1, 5, 1, 2, 3, 1, 5, 1, 3, 2, 1, 3, 1, 1,
    4, 1, 1, 1, 1, 2, 1, 2, 1, 1, 2, 1, 1, 1, 4, 4, 1, 5, 1, 1, 3, 5, 1, 1, 5, 1, 4, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 2, 2, 3, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 5, 1, 1, 1, 1, 4, 1, 1, 1,
    1, 4, 1, 1, 1, 1, 3, 1, 2, 1, 2, 1, 3, 1, 3, 4, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1, 1, 1, 1,
    4, 1, 1, 2, 2, 1, 2, 4, 1, 1, 3, 1, 1, 1, 5, 1, 3, 1, 1, 1, 5, 5, 1, 1, 1, 1, 2, 3, 4, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 4, 3, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 3, 3, 1, 2, 2, 1, 4, 1, 5, 1, 5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 5, 1, 1, 1, 4, 3, 1, 1, 4,
];

const TIMER_MAX: u32 = 8;

struct LanternfishPopulation {
    num_fish_per_reproduction_timer: [u128; (TIMER_MAX + 1) as usize],
}

impl LanternfishPopulation {
    fn from_individual_timers(timers: &[u32]) -> LanternfishPopulation {
        let mut num_fish_per_reproduction_timer = [0; (TIMER_MAX + 1) as usize];
        for &timer in timers {
            num_fish_per_reproduction_timer[timer as usize] += 1;
        }

        LanternfishPopulation {
            num_fish_per_reproduction_timer,
        }
    }

    fn size(&self) -> u128 {
        self.num_fish_per_reproduction_timer.iter().sum()
    }

    fn tick(&mut self) {
        let mut new_num_fish_per_reproduction_timer = [0; (TIMER_MAX + 1) as usize];
        for t in 0..self.num_fish_per_reproduction_timer.len() {
            let num_t_fish = self.num_fish_per_reproduction_timer[t];
            if t == 0 {
                new_num_fish_per_reproduction_timer[6] += num_t_fish;
                new_num_fish_per_reproduction_timer[8] += num_t_fish;
            } else {
                new_num_fish_per_reproduction_timer[t - 1] += num_t_fish;
            }
        }
        self.num_fish_per_reproduction_timer = new_num_fish_per_reproduction_timer;
    }
}

pub fn solution_1() -> String {
    let mut population = LanternfishPopulation::from_individual_timers(&INPUT);
    for _ in 0..80 {
        population.tick();
    }
    population.size().to_string()
}

pub fn solution_2() -> String {
    let mut population = LanternfishPopulation::from_individual_timers(&INPUT);
    for _ in 0..256 {
        population.tick();
    }
    population.size().to_string()
}
