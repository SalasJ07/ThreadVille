
fn my_mutex_init() {
    return 0
}

fn atomic_xchg(mut lock: u32) -> u32{
    mut let tmp: u32 = 1;
    asm!("xchgl $0, $1;\n" : "=r" (temp), "+*m" (&mut *lock) : "0"(temp) : "memory");
    return tmp;
}

fn test_set(mut lock:u32) -> u32 {
    atomic_xchg(lock)
}

fn destroy_my_mutex(mut lock: u32) {
    free(lock);
}

fn lock_my_mutex(mut lock: u32) {

    while *lock != 0 {
        sleep(1); 
    }
    test_set(lock);
}

fn unlock_my_mutex(mut lock: u32) {

    let mut tmp: u32 = 0
    asm!("xchgl $0, $1;\n" : "=r" (temp), "+*m" (&mut *lock) : "0"(temp) : "memory");
}

fn trylock_my_mutex(mut lock:u32) {

    while *lock != 0 {
        usleep(1000);
    }
    test_set(lock);
}