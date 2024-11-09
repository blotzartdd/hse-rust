use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use threadpool::ThreadPool;

#[test]
fn simple_test() {
    let pool = ThreadPool::new(3);
    let job = || {
        println!("Hello, world");
        let ts = time::Duration::from_millis(10);
        thread::sleep(ts);
    };
    for _ in 0..3 {
        pool.execute(job);
    }
}

#[test]
fn check_threads_cnt_test() {
    let threads_cnt = 3;
    let pool = ThreadPool::new(threads_cnt);

    let data = Arc::new(Mutex::new(HashSet::new()));
    let jobs_cnt = 30;

    for _ in 0..jobs_cnt {
        let cloned_data = Arc::clone(&data);
        let job = move || {
            let thread_id = thread::current().id();
            let ts = time::Duration::from_millis(10);
            let mut used_threads = cloned_data.lock().unwrap();
            used_threads.insert(thread_id);
            thread::sleep(ts);
        };
        pool.execute(job);
    }
    drop(pool);

    assert_eq!(
        data.lock().unwrap().len(),
        threads_cnt,
        "Your pool exploits not a {} threads to run",
        threads_cnt
    );
}

#[test]
fn threads_msgs_test() {
    let threads_cnt = 3;
    let pool = ThreadPool::new(threads_cnt);

    let data = Arc::new(Mutex::new(HashMap::new()));
    let error = Arc::new(Mutex::new(false));
    let jobs_cnt = 30;

    for _ in 0..jobs_cnt {
        let job_data = Arc::clone(&data);
        let job_error = Arc::clone(&error);
        let job = move || {
            let thread_id = thread::current().id();
            let thread_name = thread::current()
                .name()
                .expect("Thread name is missing")
                .to_string();
            let ts = time::Duration::from_millis(10);
            let mut used_threads = job_data.lock().unwrap();
            if let Some(prev_thread_id) = used_threads.get(&thread_name) {
                if *prev_thread_id != thread_id {
                    *job_error.lock().unwrap() = true;
                }
            } else {
                used_threads.insert(thread_name, thread_id);
            }
            thread::sleep(ts);
        };
        pool.execute(job);
    }
    drop(pool);

    let job_msgs = data.lock().unwrap();
    for id in 0..threads_cnt {
        assert!(
            job_msgs.contains_key(&format!("Task {}", id)),
            "Thread with name \"Task {}\" is missing",
            id
        );
    }
}

#[test]
fn long_test() {
    let threads_cnt = 5;
    let pool = ThreadPool::new(threads_cnt);

    let data = Arc::new(Mutex::new(HashSet::new()));
    let jobs_cnt = 10;

    for _ in 0..jobs_cnt {
        let cloned_data = Arc::clone(&data);
        let job = move || {
            let thread_id = thread::current().id();
            let ts = time::Duration::from_millis(500);
            let mut used_threads = cloned_data.lock().unwrap();
            used_threads.insert(thread_id);
            thread::sleep(ts);
        };
        pool.execute(job);
    }
    drop(pool);

    assert_eq!(
        data.lock().unwrap().len(),
        threads_cnt,
        "Your pool does not wait for the threads to end",
    );
}
