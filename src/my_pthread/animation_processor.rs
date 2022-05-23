use ncurses::*;

/*
    Estructuras de datos que pueden servir para el procesamiento
    de las figuras y animaciones
 */


// Para guardar la informacion de la pantalla en la que desplegar animaciones
struct screen_data {
    screen_num: i32,
    screen_width: i32,
    screen_height: i32
    let window: *mut ncurses::WINDOW, // WINDOW es de ncurses
    let preceding: *mut ncurses::screen_data,
    let next: *mut ncurses::screen_data,
}


// Para tener una serie de pantallas e ir cambiandolas
struct screen_queue {
    let head: *mut ncurses::screen_data,
    let tail: *mut ncurses::screen_data,
}



// Informacion que puede tener una animacion
struct animation_data {
    sched: i32,
    screen_num: i32,
    start: i32,
    end: i32, 
    current_x: i32,
    current_y: i32,
    starting_x: i32,
    starting_y: i32,
    ending_x: i32,
    ending_y: i32,
}

// Estructura a mostrar durante la ejecución
struct layout {
    let p: *mut String,
    total_screens: i32,
    let list: mut* ncurses::screen_queue,
}

fn animation_processor() {

}














