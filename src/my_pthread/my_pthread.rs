use signal_stack::*;
use libc::*;

// ====== Funciones de y_pthread ======

let THREADS_NUM = 1000;
let STACK_SIZE = 10000;

// Para el Context
let mut threads: [ucontext_t; 1000] 
let current_thread: *mut i64;
let mut exit_context : ucontext_t;

let mut signal_stack: signal_stack;

// Otras variables
let mut threads_off: [i64; THREADS_NUM];
let mut current_context: i64;
let mut priority:[i64; THREADS_NUM];
let mut priority_aux:[i64; THREADS_NUM];
let mut tickets:[i64; THREADS_NUM];
let mut boolean_dead_threads:[i64; THREADS_NUM];
let mut current_context: i64;
let mut init: i64;
let mut active_threads: i64;
let mut active_threads_aux: i64;
let mut total_tickets: i64;


fn set_thread_context(){


	let mut i: i64;

	// Inicializa en 0 los dead_threads
    for(i = 0; i < NUM_THREADS; i++) boolean_dead_threads[i] = 0;

    set_exit_context();
    
    // Struct necesario para la creacion del quantum
    let mut it: itimerval;

    signal_stack = malloc(STACK_SIZE);

    it.it_interval.tv_sec = 0;
    it.it_interval.tv_usec = INTERVAL * 1000; // Indica el tiempo de milisegundos 
                                              // para intervales de ejecucion
    it.it_value = it.it_interval; // Se asigna el valor

    setitimer(ITIMER_REAL, &it, null);

    let mut sigaction act;
    act.sa_sigaction = sched_alternator;

    sigemptyset(&act.sa_mask);
    act.sa_flags = SA_RESTART | SA_SIGINFO;

    sigemptyset(&set);

    sigaddset(&set, SIGALRM);

    sigaction(SIGALRM, &act, null);

}

fn execute_exit_context() {
    boolean_dead_threads[current_context] = 1;
    total_tickets -= tickets[current_context];
    active_threads_aux--;

    sched_alternator();

    while(1);
}

fn set_exit_context() {
    let mut exit_context_created: i64;
    if(!exit_context_created){
        //getcontext(ucp: *mut ucontext_t)
        getcontext(&exit_context);

        exit_context.uc_link = 0;
        exit_context.uc_stack.ss_sp = malloc(STACK_SIZE);
        exit_context.uc_stack.ss_size = STACK_SIZE;
        exit_context.uc_stack.ss_flags= 0;

        makecontext(&exit_context, &execute_exit_context, 0);

        exit_context_created = 1;
    }

}
//f: &dyn Fn(i32) -> i32
fn my_thread_create(please_work: &dyn Fn(), *mut args: std::ptr::null_mut(), tickets_s: i64, priority_s: i64) {
   
    if (!init) {
        set_thread_context();
        init++;
    }

    let *mut stack: std::ptr::null_mut(); // para utilizar context

    // Crea objeto tipo context
    let *mut thread:ucontext_t = &threads[active_threads];
    getcontext(thread);

    // Asigna memoria a context
    stack = malloc(STACK_SIZE);

    // Asigna valores por defecto
    thread -> uc_stack.ss_sp = stack;
    thread -> uc_stack.ss_size = STACK_SIZE;
    thread -> uc_stack.ss_flags = 0;
    thread -> uc_link = &exit_context;

    // Inicializa y vacia un signal set
    sigemptyset(&thread -> uc_sigmask);

    // Se manda la funcion al context
    makecontext(thread, please_work, 1, args);

    tickets[active_threads] = tickets_s;
    priority[active_threads] = priority_s;
    total_tickets += tickets_s;
    active_threads++;
    active_threads_aux++;
}

// Función encargada de terminar un thread
fn my_thread_end() {
    
    /*
    Referencia de phtread_exit()
    https://man7.org/linux/man-pages/man3/pthread_exit.3.html
    */
    
    boolean_dead_threads[current_context] = 1;
    total_tickets-=tickets[current_context];
    active_threads_aux--;

    sched_alternator();

}

fn my_thread_yield() {


    // Se usaría la función de alternar que para ceder la ejecución
    // del siguiente hilo utilizando otra implementación de scheduler
    alternate_scheduler();

}

fn my_thread_join(*mut active_thread: ucontext_t, *mut waiting_thread: ucontext_t) {

    active_thread.uc_link = waiting_thread;

}

fn my_thread_detach(*mut thread_to_detach: ucontext_t) {

    setcontext(thread_to_detach);
	free(thread_to_detach);

}

// Función que se encarga de ejecutar los hilos una vez que son creados.
// Empezando de 0 (el primero de la lista).
fn run_threads() {

    current_thread = &threads[0];

    setcontext(&threads[0]);

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