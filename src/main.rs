mod work_manager;


fn custom_test(a: u32) -> u32 {
    return a * 3;
}

fn main() {
    println!("I am just a main fn");
    let trs: i32 = 2;

    // just learning :)
    let wm: work_manager::WorkManager = work_manager::WorkManager::new(trs);

    let mut data: Vec<u32> = Vec::new();
    data.push(1);
    data.push(2);
    data.push(3);
    data.push(4);
    data.push(5);
    let vc: Vec<u32> = wm.run_job(&data, custom_test);
    println!("{:?}", vc);
    println!("DONE");
}