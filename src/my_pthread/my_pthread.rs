use nix::ucontext::UContext

// ====== Funciones de y_pthread ======

let THREADS_NUM = 1000;

// Para el Context
let mut threads: [UContext; 1000] 
let current_thread: *mut i32;
let mut exit_context : UContext;


// Otras variables
let threads_off: [THREADS_NUM];
let current_context: i32;

fn my_thread_create() {
   
    /*
    Referencia de pthread_create()
    https://man7.org/linux/man-pages/man3/pthread_create.3.html
    */
}

// Función encargada de terminar un thread
fn my_thread_end() {
    
    /*
    Referencia de phtread_exit()
    https://man7.org/linux/man-pages/man3/pthread_exit.3.html
    */
    
    threads_off[current_context] = 1;
    // La idea es que cuando termine un hilo, empiece otro con un
    // scheduler distinto

}

fn my_thread_yield() {

    /*
    Referencia de pthread_yield()
    https://man7.org/linux/man-pages/man3/pthread_yield.3.html
    */

    // Se usaría la función de alternar que para ceder la ejecución
    // del siguiente hilo utilizando otra implementación de scheduler
    alternate_scheduler();

}

fn my_thread_join() {

    /*
    Referencia de pthread_join()
    https://man7.org/linux/man-pages/man3/pthread_join.3.html
    */

}

fn my_thread_detach() {

    /*
    Referencia de pthread_detach()
    https://man7.org/linux/man-pages/man3/pthread_detach.3.html
    */

}

// Función que se encarga de ejecutar los hilos una vez que son creados.
// Empezando de 0 (el primero de la lista).
fn run_threads() {

    // Una vez ejecutados, se les asigna el context
    set(&threads[0]);
}



// ====== Funciones de my_scheduler ====== 


fn my_thread_chsched() {

}

fn run_all_threads() {

}

fn sched_round_robin() {

}

fn sched_sort() {

}

fn sched_real_time() {

}

fn alternate_scheduler() {
    
}