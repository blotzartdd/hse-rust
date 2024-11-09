use sequential::sequential_run;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{Thread, ThreadId};
use std::time::{Duration, Instant};

#[test]
fn simple_test() {
    fn natural_numbers_test(n: isize, odd_ms: u64, even_ms: u64, max_iters: isize) {
        struct ThreadData {
            id: ThreadId,
            name: String,
            last_element: i32,
        }
        let last_thread: Arc<Mutex<[Option<ThreadData>; 2]>> = Arc::new(Mutex::new([None, None]));
        let printer = move |thread: &Thread, elem: i32| {
            let mut last_thread_data = last_thread.lock().unwrap();
            let cur_idx: usize = (elem % 2).try_into().unwrap();
            let prev_idx: usize = ((elem + 1) % 2).try_into().unwrap();
            if elem >= max_iters as i32 {
                panic!("You exceeded max_iters iterations");
            }
            match &mut last_thread_data[cur_idx] {
                None => {
                    if cur_idx == 0 && !last_thread_data[prev_idx].is_none()
                        || cur_idx == 1 && last_thread_data[prev_idx].is_none()
                    {
                        panic!("You started with an odd thread");
                    }
                    last_thread_data[cur_idx] = Some(ThreadData {
                        id: thread.id(),
                        name: thread.name().unwrap().to_string(),
                        last_element: elem,
                    });
                }
                Some(last_thread_data)
                    if last_thread_data.id != thread.id()
                        || last_thread_data.name != thread.name().unwrap() =>
                {
                    panic!("You generated {elem} with the wrong thread");
                }
                Some(new_thread_data) => {
                    new_thread_data.last_element = elem;
                }
            }
        };

        let start = Instant::now();
        if n < 0 {
            sequential_run(
                printer,
                (0..).into_iter(),
                odd_ms,
                even_ms,
                max_iters.try_into().unwrap(),
            );
        } else {
            sequential_run(
                printer,
                (0..n as i32).into_iter(),
                odd_ms,
                even_ms,
                max_iters.try_into().unwrap(),
            );
        }
        let elapsed_time = start.elapsed();

        let iter_time = odd_ms.max(even_ms);
        let iterations = n.min(max_iters) as u64;
        if iter_time > 100 && iterations > 10 {
            assert!(elapsed_time <= Duration::from_millis(iter_time * iterations));
        }
    }

    natural_numbers_test(10, 1, 1, 100);
    natural_numbers_test(10, 1, 1, 5);
    natural_numbers_test(-1, 1, 1, 5);
    natural_numbers_test(5, 10, 20, 100);
    natural_numbers_test(10, 100, 2, 100);
    natural_numbers_test(10, 1000, 2000, 0);
    natural_numbers_test(20, 100, 150, 20);
    natural_numbers_test(12, 150, 150, 20);
}

#[test]
fn complex_test() {
    let result = Arc::new(Mutex::new(HashMap::from([
        (String::from("odd thread"), String::new()),
        (String::from("even thread"), String::new()),
    ])));
    let substrings = Arc::clone(&result);
    let f = move |thread: &Thread, elem: char| {
        let thread_name = thread.name().unwrap().to_string();
        let mut data = substrings.lock().unwrap();
        data.get_mut(&thread_name).unwrap().push(elem);
    };
    let example: &str = "Привет, мир";
    sequential_run(f, example.chars(), 10, 10, 100);

    let result = &*result.lock().unwrap();
    assert_eq!(result.get("odd thread"), Some(&"рвт и".to_owned()));
    assert_eq!(result.get("even thread"), Some(&"Пие,мр".to_owned()));
}
