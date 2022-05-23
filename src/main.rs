#[path = "./my_pthread/my_pthread.rs"] mod my_pthread;

fn multiXdos(numero:i64) {
    numero = numero * 2
}

fn main() {
    
    // Se llama a la función encargada de crear el thread
    my_pthread::my_thread_create(multiXdos, 2, 1, 1);
    
    my_pthread::run_all_threads()

}
