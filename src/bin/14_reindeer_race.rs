struct Reindeer {
    fly_vel: u32,
    fly_time: u32,
    rest_time: u32,
}

#[derive(Clone, Default)]
struct ReindeerState {
    dist: u32,
    points: u32,
    resting: bool,
    time_in_state: u32,
}

fn race(reindeer: &[Reindeer], time: u32) -> Option<(u32, u32)> {
    if reindeer.is_empty() {
        return None;
    }

    let mut states = vec![ReindeerState::default(); reindeer.len()];
    let mut max_dist = 0;

    for _ in 0..time {
        for (r, state) in reindeer.iter().zip(states.iter_mut()) {
            if !state.resting {
                state.dist += r.fly_vel;
                max_dist = std::cmp::max(max_dist, state.dist);
            }

            state.time_in_state += 1;
            let limit = if state.resting {
                r.rest_time
            } else {
                r.fly_time
            };
            if state.time_in_state == limit {
                state.resting = !state.resting;
                state.time_in_state = 0;
            }
        }

        for state in states.iter_mut() {
            if state.dist == max_dist {
                state.points += 1;
            }
        }
    }

    let dists = states.iter().map(|s| s.dist);
    let points = states.iter().map(|s| s.points);

    Some((dists.max().unwrap(), points.max().unwrap()))
}

fn main() {
    let reindeer = adventofcode::read_input_lines(|line| {
        let nums = adventofcode::numbers(line);
        Reindeer {
            fly_vel: nums[0],
            fly_time: nums[1],
            rest_time: nums[2],
        }
    });

    let (dist, points) = race(&reindeer, 2503).unwrap();

    println!("{}", dist);
    println!("{}", points);
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test_race {
            simple(&[
                 Reindeer { fly_vel: 1000, fly_time: 1, rest_time: 0 },
            ], 1, Some((1000, 1)));
            rest_start(&[
                 Reindeer { fly_vel: 1000, fly_time: 2, rest_time: 1 },
            ], 3, Some((2000, 3)));
            rest_end(&[
                 Reindeer { fly_vel: 1000, fly_time: 2, rest_time: 1 },
            ], 4, Some((3000, 4)));
            aoc_given(&[
                 Reindeer { fly_vel: 14, fly_time: 10, rest_time: 127 },
                 Reindeer { fly_vel: 16, fly_time: 11, rest_time: 162 },
            ], 1000, Some((1120, 689)));
        }
    }

    fn test_race(s: &[Reindeer], time: u32, expect: Option<(u32, u32)>) {
        assert_eq!(race(s, time), expect);
    }
}
