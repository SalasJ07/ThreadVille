//extern crate ncurses;

use ncurses::*;

/*
    ncurses funciona para aplicaciones de consola, tambien se 
    pueden cambiar colores o mover el cursor
*/


// Estructuras de datos que pueden servir para el procesamiento
// de las figuras y animaciones


/*
    Structs en Rust:
    https://doc.rust-lang.org/book/ch05-01-defining-structs.html
*/

// Para guardar la informacion de la pantalla en la que desplegar animaciones
struct screen_data {
    screen_num: u32,
    screen_width: u32,
    screen_height: u32
    let window: *mut WINDOW, // WINDOW es de ncurses
    let preceding: *mut screen_data,
    let next: *mut screen_data,
}


// Para tener una serie de pantallas e ir cambiandolas

struct screen_queue {
    let head: *mut screen_data,
    let tail: *mut screen_data,
}



// Informacion que puede tener una animacion

struct animation_data {
    sched: u32,
    screen_num: u32,
    start: u32,
    end: u32, 
    current_x: u32,
    current_y: u32,
    starting_x: u32,
    starting_y: u32,
    ending_x: u32,
    ending_y: u32,
    // -
}


struct layout {
    let p: *mut String,
    total_screens: u32,
    let list: mut* screen_queue,
    // -
}

fn animation_processor() {

}













