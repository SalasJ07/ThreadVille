/** my_mutex_init
 * Reciba un a variable lock
 * asigna un valor 0 a lock, basicamente inicializa el lock/mutex
 */
fn my_mutex_init(*mut lock) {
    lock = 0
}
/** atomic_xchg
 * Recibe un lock
 * Genera el espacio del mutex
 */
fn atomic_xchg(*mut lock: i64) -> i64{
    mut let tmp: i64 = 1;
    asm!("xchgl $0, $1;\n" : "=r" (temp), "+*m" (&mut *lock) : "0"(temp) : "memory");
    return tmp;
}
/** set_test 
 * Recibe el Lock
 * No regresa nada
 * Genera el blockeo del lock
*/
fn set_test(*mut lock:i64) -> i64 {
    atomic_xchg(lock)
}

/** my_mutex_destroy 
 * Recibe el Lock
 * No regresa nada
 * Libera al lock
*/
fn my_mutex_destroy(*mut lock: i64) {
    free(lock);
}

/** my_mutex_lock 
 * Recibe el Lock
 * No regresa nada
 * Bloquea el mutex
*/
fn my_mutex_lock(*mut lock: i64) {

    while *lock != 0 {
        sleep(1); 
    }
    set_test(lock);
}

/** my_mutex_unlock 
 * Recibe el Lock
 * No regresa nada
 * Libera el lock
*/
fn my_mutex_unlock(*mut lock: i64) {

    let mut tmp: i64 = 0
    asm!("xchgl $0, $1;\n" : "=r" (temp), "+*m" (&mut *lock) : "0"(temp) : "memory");
}

/** my_mutex_trylock
 * Recibe el Lock
 * No regresa nada
 * Bloquea el lock durante un cierto tiempo
 */
fn my_mutex_trylock(*mut lock:i64) {

    while *lock != 0 {
        usleep(1000);
    }
    set_test(lock);
}