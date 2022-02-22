use std::vec::Vec;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use rand::distributions::Uniform;
use rand::{Rng};


pub struct WorkManager {
    pub threshold: i32,
}


impl WorkManager {
    pub fn new(trs: i32) -> Self {
        Self{threshold: trs}
    }

    pub fn run_job(&self, input: & Vec<u32>, f: fn(u32) -> u32) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        if input.len() > self.threshold as usize {
            // split data and run dispatch multiple
            result = self.dispatch_multiple(input, f);
        } else {
            result = self.dispatch_single(input, f);
        }
        return result
    }

    fn dispatch_single(&self, input: & Vec<u32>, f: fn(u32) -> u32) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        for x in input {
            result.push(f(*x));
        }
        return result
    }

    fn dispatch_multiple(&self, input: & Vec<u32>, f: fn(u32) -> u32) -> Vec<u32> {
        // let mut result: Vec<u32> = Vec::new();
        let chunks: Vec<Vec<u32>> = self.split_data(input);
        let mut result = Arc::new(Mutex::new(vec![0; input.len() as usize]));
        let trs: usize = self.threshold as usize;
        let mut children = vec![];
        for i in 0..chunks.len() {
            let result = result.clone();
            let chunks = chunks.clone();
            children.push(thread::spawn(move || {
                for j in 0..chunks[i].len() {
                    result.lock().unwrap()[i * trs + j] = f(chunks[i][j]);
                }
            }));
        }

        for c in children {
            let _ = c.join();
        }
        let mut f_result: Vec<u32> = vec![0; input.len() as usize];
        for i in 0..input.len() {
            f_result[i] = result.lock().unwrap()[i];
        }
        return f_result;
    }

    fn split_data(&self, input: & Vec<u32>) -> Vec<Vec<u32>> {
        let mut chunks: Vec<Vec<u32>> = Vec::new();
        let chunk_size: i32 = self.threshold;
        let mut i = 0;
        while i < input.len() {
            let mut end = i + chunk_size as usize;
            if end > input.len() {
                end = input.len();
            }
            let mut j = i;
            let mut chunk: Vec<u32> = Vec::new();
            while j < end {
                chunk.push(input[j]);
                j += 1;
            }
            chunks.push(chunk);
            i += chunk_size as usize;
        }
        return chunks;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TRS_TEST: i32 = 10;

    fn testing_function(a: u32) -> u32 {
        return 3 * a;
    }

    #[test]
    fn test_single() {
        let wm: WorkManager = WorkManager::new(TRS_TEST);
        let size = TRS_TEST - 1;
        let range = Uniform::from(0..20);
        let values: Vec<u32> = rand::thread_rng().sample_iter(&range).take(size as usize).collect();

        // actual result
        let actual_result: Vec<u32> = wm.run_job(&values, testing_function);
        // expected result
        let mut expected_result: Vec<u32> = Vec::new();
        for c in values {
            expected_result.push(testing_function(c));
        }

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_multiple() {
        let wm: WorkManager = WorkManager::new(TRS_TEST);
        let size = TRS_TEST*2;
        let range = Uniform::from(0..20);
        let values: Vec<u32> = rand::thread_rng().sample_iter(&range).take(size as usize).collect();


        let actual_result: Vec<u32> = wm.run_job(&values, testing_function);

        let mut expected_result: Vec<u32> = Vec::new();
        for c in values {
            expected_result.push(testing_function(c));
        }
        assert_eq!(actual_result, expected_result);
    }
}