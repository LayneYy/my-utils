use my_utils::thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

#[test]
fn test_pool() {
    let pool = ThreadPool::new(10);
    for i in 1..=10 {
        pool.execute(move || {
            println!("{} is sleeping.", i);
            thread::sleep(Duration::from_secs( i*1));
            println!("{} week up.", i);
        });
    }
}