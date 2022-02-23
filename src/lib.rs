use rand::distributions::Uniform;
use rand::Rng;
use std::vec::Vec;
use std::thread::{self, JoinHandle};

pub struct WorkManager {
    pub threshold: i32,
}

impl WorkManager {
    pub fn new(trs: i32) -> Self {
        Self { threshold: trs }
    }

    pub fn run_job<T: Send + Sync>(&self, input: &'static Vec<T>, f: fn(&T) -> T) -> Vec<T> {
        if input.len() > self.threshold as usize {
            let mut guards: Vec<JoinHandle<Vec<T>>> = vec!();
            for chunk in input.chunks(self.threshold as usize) {
                let chunk = chunk.to_owned();
                let g = thread::spawn(move || chunk.iter().map(|x| f(x)).collect());
                guards.push(g);
            };
            let mut result: Vec<T> = Vec::with_capacity(input.len());
            for g in guards {
                result.extend(g.join().unwrap().into_iter());
            }
            result
        } else {
            return input.iter().map(|x| f(x)).collect::<Vec<T>>();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TRS_TEST: i32 = 10;

    fn testing_function(a: &u32) -> u32 {
        return 3 * a;
    }

    #[test]
    fn test_single() {
        let wm: WorkManager = WorkManager::new(TRS_TEST);
        let values: Vec<u32> = rand::thread_rng()
            .sample_iter(&Uniform::from(0..20))
            .take(TRS_TEST as usize - 1)
            .collect();
        assert_eq!(
            wm.run_job(&values, testing_function),
            values
                .iter()
                .map(|x| testing_function(x))
                .collect::<Vec<u32>>()
        );
    }

    #[test]
    fn test_multiple() {
        let wm: WorkManager = WorkManager::new(TRS_TEST);
        let values: Vec<u32> = rand::thread_rng()
            .sample_iter(&Uniform::from(0..20))
            .take(TRS_TEST as usize * 2)
            .collect();

        assert_eq!(
            wm.run_job(&values, testing_function),
            values
                .iter()
                .map(|x| testing_function(x))
                .collect::<Vec<u32>>()
        );
    }
}
