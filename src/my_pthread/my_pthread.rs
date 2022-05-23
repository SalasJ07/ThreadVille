use signal_stack::*;
use libc::*;

// ====== Funciones de y_pthread ======

const THREADS_NUM: u64 = 1000;
const STACK_SIZE: u64 = 10000;

// Para el Context
static mut threads: libc::ucontext_t = 1000; 
static current_thread: *mut i64;
static mut exit_context : libc::ucontext_t;

static mut signal_stack: signal_stack::signal_stack;

// Variables relacionadas a los hilos
static mut threads_off:[i64; THREADS_NUM] = [0,THREADS_NUM];
static mut current_context: i64 = 0;
static mut priority:[i64; THREADS_NUM] = [0,THREADS_NUM];
static mut priority_aux:[i64; THREADS_NUM] = [0,THREADS_NUM];
static mut tickets:[i64; THREADS_NUM] = [0,THREADS_NUM];
static mut dead_threads:[i64; THREADS_NUM] = [0,THREADS_NUM];
static mut current_context: i64 = 0;
static mut initialize: i64 = 0;
static mut active_threads: i64 = 0;
static mut active_threads_aux: i64 = 0;
static mut total_tickets: i64 = 0;

/**
 * set_thread_context
 * No recibe ningun parametros
 * Se utiliza para inicializar las variables necesarias para crear contexto
 * Solo se utilizará al crear el primer hilo
 * No retorna nada
 */
pub fn set_thread_context(){


	let mut i: i64;

	// Inicializa en 0 los dead_threads
    for i in NUM_THREADS {
        dead_threads[i] = 0;
    } 

    //sale del contexto actual
    set_exit_context();
    
    let mut it: libc::itimerval;

    signal_stack = libc::malloc(STACK_SIZE);

    it.it_interval.tv_sec = 0;
    it.it_interval.tv_usec = INTERVAL * 1000; // Tiempo en minisegundos
    it.it_value = it.it_interval;

    setitimer(libc::ITIMER_REAL, &it, null);

    let mut act: libc::sigaction;
    act.sa_sigaction = sched_alternator;

    libc::sigemptyset(&act.sa_mask);
    act.sa_flags = libc::SA_RESTART | libc::SA_SIGINFO;

    libc::sigemptyset(&libc::set);

    libc::sigaddset(&libc::set, libc::SIGALRM);

    libc::sigaction(libc::SIGALRM, &act, null);

}
/** execute_exit_context 
 * No recibe parametros
 * No retorna nada
 * Metodo utilizado para eliminar o finalizar un contexto
 * Selecciona en la variable dead_threads que el contexto finalizo asignandole un 1
 * alterna el sched utilizado
 * elimina de los contadores el contexto debido a que ya finalizó
 */
pub fn execute_exit_context() {
    dead_threads[current_context] = 1;
    total_tickets -= tickets[current_context];
    active_threads_aux -= 1;

    sched_alternator();

    while(1);
}
/** set_exit_context
 * No recibe parametros
 * No retorna nada
 * Le indica al sistema que finalice el contexto
 * Además llama al metood execute_exit_context para actualizar las variables del programa
 */
pub fn set_exit_context() {
    let mut exit_context_created: i64;
    if !exit_context_created {
        libc::getcontext(&exit_context);

        exit_context.uc_link = 0;
        exit_context.uc_stack.ss_sp = malloc(STACK_SIZE);
        exit_context.uc_stack.ss_size = STACK_SIZE;
        exit_context.uc_stack.ss_flags= 0;

        makecontext(&exit_context, &execute_exit_context, 0);

        exit_context_created = 1;
    }

}

/** my_thread_create
 * Recibe como parametros la funcion a correr, los argumentos para dicha funcion, numero de ticket, su prioridad
 * No retorna nada
 * Este metodo se encarga de crear un hilo para la funcion que se requiera
 * Le asigna espacio de memoria, además de inicializar el hilo y agregarlo a la lista de hilos de la lib
 */
pub fn my_thread_create(please_work: &dyn Fn(), *mut args: std::ptr::null_mut(), tickets_s: i64, priority_s: i64) {
   
    if !initialize {
        set_thread_context();
        initialize += 1;
    }

    let stack: *mut std::ptr::null_mut(); // para utilizar context

    // Crea objeto tipo context
    let thread : *mut libc::ucontext_t = &threads[active_threads];
    libc::getcontext(thread);

    // Asigna memoria a context
    stack = libc::malloc(STACK_SIZE);

    // Asigna valores por defecto
    thread.uc_stack.ss_sp = stack;
    thread.uc_stack.ss_size = STACK_SIZE;
    thread.uc_stack.ss_flags = 0;
    thread.uc_link = &exit_context;

    // Inicializa y vacia un signal set
    libc::sigemptyset(&thread.uc_sigmask);

    // Se manda la funcion al context
    libc::makecontext(thread, please_work, 1, args);

    tickets[active_threads] = tickets_s;
    priority[active_threads] = priority_s;
    total_tickets += tickets_s;
    active_threads += 1;
    active_threads_aux += 1;
}
/** my_thread_end
 * No recibe parametros
 * No retorna nada
 * Este metodo se encarga de terminar un thread
 * además actualiza las variables
 */
pub fn my_thread_end() {
    
    
    dead_threads[current_context] = 1;
    total_tickets -= tickets[current_context];
    active_threads_aux -= 1;

    sched_alternator();

}
/** my_thread_yield
 * No recibe pametros
 * No retorna nada
 * Función encargada de alternar el scheduler
 */
pub fn my_thread_yield() {

    alternate_scheduler();

}

/** my_thread_join
 * Recibe 2 contextos
 * no retorna nada
 * Este metodo se encarga de modificar el enlace del primer hilo, asignandole el segundo hilo
 */
pub fn my_thread_join(active_thread: *mut libc::ucontext_t, *mut waiting_thread: libc::ucontext_t) {

    active_thread.uc_link = waiting_thread;

}

/** my_thread_detach
 * Recibe un hilo para desactivar
 * No retorna nada
 * libera un contexto o hilo
 */
pub fn my_thread_detach(thread_to_detach: *mut libc::ucontext_t) {

    libc::setcontext(thread_to_detach);
	libc::free(thread_to_detach);

}

/** run_all_threads
 * No recibe nada
 * No regresa nada
 * Metodo encargado de ejecutar el primer hilo de la lista
 */
pub fn run_all_threads() {

    current_thread = &threads[0];

    libc::setcontext(&threads[0]);

}

// ====== Funciones de my_scheduler ====== 

static mut active_sched: i64 = 0;
static mut signal_context: libc::ucontext_t;
static mut alternate: i64 = 0;
/** my_thread_chsched
 * recibe cual sera el nuevo sched
 * no retorna nada
 * depende del valor ingresado se le asigna ese valor
 */
pub fn my_thread_chsched(new_sched: i64) {
    if new_sched == 0 {
        active_sched = 0;
    } else if new_sched == 1 {
        active_sched = 1;
    } else if new_sched == 2 {
        active_sched = 2;
    }
}

/** sched_round_robin
 * No recibe nada
 * No retorna nada
 * Metodo utilizado para cambiar de hilo, este metodo busca apartir del hilo actual el siguiente hilo activo
 * Por medio de la lista dead_threads verifica que el hilo no se haya terminado en caso de
 * no ser un hilo finalizado lo asigna y lo corre
 */
pub fn sched_round_robin() {
    if active_threads_aux > 0 {

        current_context = current_context + 1; // Pasa al siguiente contexto

        // Verificar si el thread sigue vivo
        if dead_threads[current_context% active_threads] {

            // Sigue revisando la lista hasta que encuentre un thread vivo
            while dead_threads[current_context% active_threads] {
                current_context+=1;
            }
        }

        // Se asigna el contexto al thread actual
        current_context = current_context % active_threads;
        current_thread = &threads[current_context];

        libc::setcontext(current_thread); //activa el nuevo hilo
    }
}

/** sched_sort
 * No recibe nada
 * No retorna nada
 * Este metodo genera un numero aleatorio el cual nos indicara cual de los hilos es el siguiente en ser ejecutado
 * una vez elegido uno se verifica que no sea un hilo finalizado
 * en caso de serlo se pasa al siguiente hilo no finalizado
 */
pub fn sched_sort() {
    //srand(time(NULL));

    let mut aux: i64;

    // Tiene la cantidad de threads que estan activos y vivos 
    // y valida si hay alguno
    if active_threads_aux > 0 {

        let mut winner:i64 = //rand()%(total_tickets+1);//saca el ganador del sorteo
        aux = winner;
        let mut i: i64;

        for i in active_threads {//revisa quien es el ganador

            aux -= tickets[i];

            if aux <= 0 {
                if dead_threads[i% active_threads] {
                    while dead_threads[i% active_threads] {
                        i += 1;
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
        libc::setcontext(current_thread);//activa el nuevo hilo
    }
}

/** sched_real_time
 * No recibe nada
 * No retorna nada
 * Metodo utilizado para ejecutar los hilos por prioridad
 * La funcion se encarga de buscar el hilo con mayor prioridad y ejecutarlo
 */
pub fn sched_real_time() {
    let mut aux: i64 = -1;
    let mut last: i64 = current_context;
    let mut i: i64;

    // Tiene la cantidad de threads que estan activos y vivos y valida si hay alguno
    if active_threads_aux > 0{

        // Itera hasta encontrar el hilo con mayor prioridad que no haya sido ejecutado
        for i in active_threads {

            if aux < priority[i] && !dead_threads[i] && !priority_aux[i] {

                current_context = i;
                aux = priority[i];
            }
        }

        if aux == -1 {

            for i in active_threads {
                priority_aux[i] = 0;
            }

            my_sched_real_time();

        }
        else{

            // Fija el thread como ya ejecutado
            priority_aux[current_context] = 1;
            current_thread = &threads[current_context];

            libc::setcontext(current_thread);
        }
    }

}
/** alternate_scheduler
 * No recibe nada
 * No retorna nada
 * Metodo utilizado para alternar el scheduler por medio de un xor entre un 0 y el sched actual
 * ademas de reiniciar los signals
 */
pub fn alternate_scheduler() {
    libc::getcontext(&signal_context);

    signal_context.uc_stack.ss_sp = signal_stack;
    signal_context.uc_stack.ss_size = STACK_SIZE;
    signal_context.uc_stack.ss_flags = 0;

    libc::sigemptyset(&signal_context.uc_sigmask);

    alternate = 0;

    // Alterna el valor de active_sched por medio de un xor
    alternate = alternate^active_sched;

    // Se envia el valor del nuevo algoritmo de scheduling a utilizar
    my_thread_chsched(alternate);

    match alternate{
        0=>libc::makecontext(&signal_context, sched_round_robin, 1),
        1=>libc::makecontext(&signal_context, sched_sort, 1),
        2=>libc::makecontext(&signal_context, sched_real_time, 1),
    }
    
    libc::swapcontext(current_thread,&signal_context);
    
}