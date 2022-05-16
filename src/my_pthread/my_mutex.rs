
fn my_mutex_init() {
    return 0
}

fn atomic_xchg(mut lock: u32) -> u32{
    mut let tmp: u32 = 1;
    asm!("xchgl $0, $1;\n" : "=r" (temp), "+*m" (&mut *lock) : "0"(temp) : "memory");
    return tmp;
}

fn set_test(mut lock:u32) -> u32 {
    atomic_xchg(lock)
}

fn my_mutex_destroy(mut lock: u32) {
    free(lock);
}

fn my_mutex_lock(mut lock: u32) {

    while *lock != 0 {
        sleep(1); 
    }
    set_test(lock);
}

fn my_mutex_unlock(mut lock: u32) {

    let mut tmp: u32 = 0
    asm!("xchgl $0, $1;\n" : "=r" (temp), "+*m" (&mut *lock) : "0"(temp) : "memory");
}

fn my_mutex_trylock(mut lock:u32) {

    while *lock != 0 {
        usleep(1000);
    }
    set_test(lock);
}