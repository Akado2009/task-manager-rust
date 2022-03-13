use rand::distributions::Uniform;
use rand::Rng;
use std::vec::Vec;
use crossbeam;

pub struct WorkManager {
    pub threshold: i32,
}

impl WorkManager {
    pub fn new(trs: i32) -> Self {
        Self { threshold: trs }
    }

    pub fn run_job<T: Send + Sync>(&self, input: &[T], f: impl Fn(&T) -> T + Sync) -> Vec<T> {
        if input.len() > self.threshold as usize {
            crossbeam::scope(|scope| {
                let mut guards = vec![];
                for chunk in input.chunks(self.threshold as usize) {
                    let g = scope.spawn(|_| chunk.iter().map(|x| f(x)).collect::<Vec<_>>());
                    guards.push(g);
                }
                guards.into_iter().flat_map(|g| g.join().unwrap()).collect()
            })
            .unwrap()
        } else {
            input.iter().map(|x| f(x)).collect()
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
        let nvalues: Vec<u32> = rand::thread_rng()
            .sample_iter(&Uniform::from(0..20))
            .take(TRS_TEST as usize * 2)
            .collect();

        assert_eq!(
            wm.run_job(&nvalues, testing_function),
            nvalues
                .iter()
                .map(|x| testing_function(x))
                .collect::<Vec<u32>>()
        );
    }
}
