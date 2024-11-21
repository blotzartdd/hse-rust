#![forbid(unsafe_code)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;
use std::time;

pub fn sequential_run<
    Printer: Fn(&Thread, Iter::Item) + Send + 'static + Clone,
    Iter: Iterator + Send + 'static,
>(
    printer: Printer,
    iterator: Iter,
    odd_thread_ms: u64,
    even_thread_ms: u64,
    max_iterations: usize,
) where
    Iter::Item: std::fmt::Debug,
{
    let mutex_iterator = Arc::new(Mutex::new(iterator));
    let counter = Arc::new(Mutex::new(0_usize));
    let printer_func = Arc::new(Mutex::new(printer.clone()));
    let iter_end = Arc::new(Mutex::new(false));

    let odd_counter = Arc::clone(&counter);
    let odd_iter = Arc::clone(&mutex_iterator);
    let odd_printer = Arc::clone(&printer_func);
    let odd_iter_end = Arc::clone(&iter_end);
    let odd_thread = thread::Builder::new()
        .name("odd thread".to_string())
        .spawn(move || loop {
            let mut count = odd_counter.lock().unwrap();
            let mut iter_end = odd_iter_end.lock().unwrap();
            if *iter_end || *count >= max_iterations {
                *iter_end = true;
                break;
            }

            if *count % 2 != 0 {
                thread::sleep(time::Duration::from_millis(odd_thread_ms / 2));

                let mut cur_iter = odd_iter.lock().unwrap();

                let cur_item_opt = cur_iter.next();
                if cur_item_opt.is_none() {
                    *iter_end = true;
                    break;
                }

                let cur_printer = odd_printer.lock().unwrap();
                cur_printer(&thread::current(), cur_item_opt.unwrap());

                *count += 1;
            }
        })
        .unwrap();

    let even_counter = Arc::clone(&counter);
    let even_iter = Arc::clone(&mutex_iterator);
    let even_printer = Arc::clone(&printer_func);
    let even_iter_end = Arc::clone(&iter_end);
    let even_thread = thread::Builder::new()
        .name("even thread".to_string())
        .spawn(move || loop {
            let mut count = even_counter.lock().unwrap();
            let mut iter_end = even_iter_end.lock().unwrap();
            if *iter_end || *count >= max_iterations {
                *iter_end = true;
                break;
            }

            if *count % 2 == 0 {
                thread::sleep(time::Duration::from_millis(even_thread_ms / 2));
                let mut cur_iter = even_iter.lock().unwrap();

                let cur_item_opt = cur_iter.next();
                if cur_item_opt.is_none() {
                    *iter_end = true;
                    break;
                }

                let cur_printer = even_printer.lock().unwrap();
                cur_printer(&thread::current(), cur_item_opt.unwrap());

                *count += 1;
            }
        })
        .unwrap();

    even_thread.join().unwrap();
    odd_thread.join().unwrap();
}
