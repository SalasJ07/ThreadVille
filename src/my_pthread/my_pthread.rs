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
    for(i = 0; i < NUM_THREADS; i += 1) boolean_dead_threads[i] = 0;

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
        init += 1;
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
    active_threads += 1;
    active_threads_aux += 1;
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
fn run_all_threads() {

    current_thread = &threads[0];

    setcontext(&threads[0]);

}

// ====== Funciones de my_scheduler ====== 

let mut active_sched: i64;
let mut signal_context: ucontext_t;
let mut alternate: i64;

fn my_thread_chsched(new_sched: i64) {
    if(new_sched == 0) {
        active_sched = 0;
    } else if(new_sched == 1) {
        active_sched = 1;
    } else if(new_sched == 2) {
        active_sched = 2;
    }
}

fn sched_round_robin() {
    if(active_threads_aux > 0){

        current_context = (current_context + 1); // Pasa al siguiente contexto

        // Verificar si el thread sigue vivo
        if(boolean_dead_threads[current_context% active_threads]){

            // Sigue revisando la lista hasta que encuentre un thread vivo
            while(boolean_dead_threads[current_context% active_threads]){
                current_context+=1;
            }
        }

        // Se asigna el contexto al thread actual
        current_context = current_context % active_threads;
        current_thread = &threads[current_context];

        setcontext(current_thread); //activa el nuevo hilo
    }
}

fn sched_sort() {
    //srand(time(NULL));

    let mut aux: i64;

    // Tiene la cantidad de threads que estan activos y vivos 
    // y valida si hay alguno
    if(active_threads_aux > 0){

        let mut winner:i64 = //rand()%(total_tickets+1);//saca el ganador del sorteo
        aux = winner;
        let mut i: i64;

        for (i = 0; i < active_threads; i += 1) {//revisa quien es el ganador

            aux -= tickets[i];

            if(aux<=0){
                if(boolean_dead_threads[i% active_threads]){
                    while(boolean_dead_threads[i% active_threads]){
                        i+=1;
                    }
                }

                current_context = i;
                current_thread = &threads[current_context];
                break;
            }

            else{

                tickets[i] += 1;
                total_tickets += 1;
            }
        }
        setcontext(current_thread);//activa el nuevo hilo
    }
}

fn sched_real_time() {
    let mut aux: i64 = -1;
    let mut last: ucontext_t = current_context;
    let mut i: i64;

    // Tiene la cantidad de threads que estan activos y vivos 
    // y valida si hay alguno
    if(active_threads_aux > 0){

        // Itera hasta encontrar el hilo con mayor prioridad que no haya sido
        // ejecutado
        for (i = 0; i < active_threads; i+=1) {

            if(aux < priority[i] && !boolean_dead_threads[i] && !priority_aux[i]){

                current_context = i;
                aux = priority[i];
            }
        }

        if(aux == -1){

            for (i = 0; i < active_threads; i+=1) {

                priority_aux[i] = 0;
            }

            my_sched_real_time();

        }
        else{

            // Fija el thread como ya ejecutado
            priority_aux[current_context] = 1;
            current_thread = &threads[current_context];

            setcontext(current_thread);
        }
    }

}

fn alternate_scheduler() {
    getcontext(&signal_context);

    signal_context.uc_stack.ss_sp = signal_stack;
    signal_context.uc_stack.ss_size = STACK_SIZE;
    signal_context.uc_stack.ss_flags = 0;

    sigemptyset(&signal_context.uc_sigmask);

    alternate = 0;

    // Alterna el valor de active_sched
    alternate = alternate^active_sched;

    // Se envia el valor del nuevo algoritmo de scheduling a utilizar
    my_thread_chsched(alternate);

    if(active_sched == 0){makecontext(&signal_context, my_sched_round_robin, 1);}

    if(active_sched == 1){makecontext(&signal_context, my_sched_sort, 1);}

    if(active_sched == 2){makecontext(&signal_context, my_sched_real_time, 1);}

    swapcontext(current_thread,&signal_context);
    
}