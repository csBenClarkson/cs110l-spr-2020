use crossbeam_channel;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use std::{thread, time};
use rand;
use rand::Rng;

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    unsafe {
        output_vec.set_len(input_vec.len());
    }

    /*
                  |--- T0 --- compute |
    dispatcher ---|--- T1 --- compute |--- collector
                  |--- T2 --- compute |
    */

    // Task: (index: usize, data: T)
    // Output: (index: usize, result_data: U)
    let (dispatcher, worker): (Sender<(usize, T)>, Receiver<(usize, T)>) = crossbeam_channel::unbounded();
    let (worked, collector): (Sender<(usize, U)>, Receiver<(usize, U)>) = crossbeam_channel::unbounded();

    // spawning worker threads
    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let this_worker = worker.clone();
        let this_worked = worked.clone();
        threads.push(thread::spawn(move || {
            while let Ok(task) = this_worker.recv() {
                let result = f(task.1);
                this_worked.send((task.0, result)).expect("sending fails.");
            }
            drop(this_worked);
        }))
    }
    drop(worked);

    // dispatching tasks
    while let Some(data) = input_vec.pop() {
        dispatcher.send((input_vec.len(), data)).expect("sending fails.");
    }
    drop(dispatcher);

    while let Ok(output) = collector.recv() {
        output_vec[output.0] = output.1;
    }

    for t in threads {
        t.join().expect("thread join fails.");
    }

    output_vec
}

fn main() {
    // let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..1000 {
        v.push(rng.gen_range(-500..500));
    }
    let v_cloned1 = v.clone();
    let v_cloned2 = v.clone();

    /* parallel_map */
    let mut start = time::Instant::now();
    let squares = parallel_map(v, num_cpus::get(), |num| {
        println!("{} squared is {}", num, num * num);
        // thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    let parallel_used = start.elapsed();

    /* raw for loop */
    start = time::Instant::now();
    let mut squares_for = Vec::new();
    for num in v_cloned1 {
        println!("{} squared is {}", num, num * num);
        squares_for.push(num * num);
    }
    let for_used = start.elapsed();

    /* iter().map() */
    start = time::Instant::now();
    let squares_map = v_cloned2.iter().map(|num| {
        println!("{} squared is {}", num, num * num);
        num * num
    }).collect::<Vec<i32>>();
    let map_used = start.elapsed();

    assert_eq!(squares_map, squares);
    assert_eq!(squares_for, squares);
    println!("squares: {:?}", squares);

    println!("\n=================================\n");
    println!("parallel_map use: {:?}", parallel_used);
    println!("for loop use: {:?}", for_used);
    println!("iter().map() use: {:?}", map_used);
}
