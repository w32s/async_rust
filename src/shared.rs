use std::ops::{BitAndAssign, Deref};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

//Action 5 B (extra)

pub async fn poisoned_mutex(){
    let shared_data = Arc::new(Mutex::new(5));
    let clone = shared_data.clone();
    let handle = thread::spawn(move ||{
        let res = clone.lock();
        if 5==5 {
            panic!("This thread will panic and poison the mutex");
        }
    });
    let handle2 = thread::spawn(move ||{
        let mut received_value = match shared_data.lock() {
            Ok(guard) => guard,
            Err(e) => panic!("Failed to receive mutex, since it was poisoned. {:?}", e)
        };
    });
    // simulate a poisoned mutex
    match handle.join(){
        Ok(_) => println!("Thread 1 finished successfully"),
        Err(e) => eprintln!("Thread 1 panicked: {:?}", e)
    }
    match handle2.join(){
        Ok(_) => println!("Thread 2 finished successfully"),
        Err(e) => eprintln!("Thread 2 panicked: {:?}", e)
    }
}

//Action 5
pub(crate) async fn sharing_data_between_threads() {
    let shared_data = Arc::new((Mutex::new("This"), Condvar::new()));
    // Tuple wrapped in an arc.
    // Shared data clone is passed to the background thread
    // original data remains on the main thread... Without cloning ownership would be transferred to the background thread,
    // and the main thread would not be able to access it.

    let shared_data_clone = Arc::clone(&shared_data);
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop);

    let _background_thread = thread::spawn(move || {
        let (lock, cvar) = shared_data_clone.deref(); // Deref the smart pointer and then take a reference to the inner value. &* syntax works too
        let mut received_value = match lock.lock() {
            Ok(guard) => guard,
            Err(e) => panic!("Failed to receive mutex, since it was poisoned. {:?}", e)
        };

        while !stop.load(Relaxed) {
            received_value = cvar.wait(received_value).unwrap();
            // Wait for the main thread to signal the background
            // thread via the condvar
            println!("Received value: {}", *received_value);
        }
        println!("background thread: STOP has been updated");
    });
    let updater_thread = thread::spawn(move || {
        let (lock, cvar) = &*shared_data;
        let values = ["is", "a", "blue", "house"];

        for i in 0..4 {
            let update_value = values[i as usize];
            println!("Updating value to {}...", update_value);
            *lock.lock().unwrap() = update_value;
            cvar.notify_one(); // wakes up the background thread try commenting this line and see what happens
            thread::sleep(Duration::from_secs(4));
        }
        stop_clone.store(true, Relaxed);
        println!("Updater thread: updating STOP");
        cvar.notify_one();
    });
    updater_thread.join().unwrap(); // block the main thread until the updater_thread has finished

    /*
    Here we see that we update the value and then notify the other thread that the value has changed.
     We then block the main program until the updater_thread has finished.

     You will notice that we have used this Relaxed term.
      It is critical to ensure that operations occur in a specific order to avoid data races and strange inconsistencies.
      This is where memory ordering comes into play.
      The Relaxed ordering, used with AtomicBool, ensures that the operations on the atomic variable are visible to all threads
      but it does not enforce any particular order on the surrounding operations.
      This is sufficient for our example because we only need to check the value of STOP
       and don’t care about strict ordering of other operations.
     */
}