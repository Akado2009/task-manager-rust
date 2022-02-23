pub struct WorkManager {
    pub threshold: i32,
}

implt WorkManager {
    pub fn new(trs: i32) -> Self {
        Self {threshold: trs}
    }

    pub fn run_job<T>(&self, input: &Vec<T>, f: fn(&T) -> T) -> Vec<T> {
        let larger = self.threshold > input.len();
        match larger {
            true => {
                let chunks = input.chunks(self.threshold);
                let mut result = Arc::new(Mutex::new(vec![0; input.len() as usize]));
            }
            false => {
                return vector.iter().map(|x| fn).collect::<Vec<T>>();
            }
        }
    }
}