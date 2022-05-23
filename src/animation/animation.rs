#[path = "./my_pthread/my_mutex.rs"] mod my_mutex;
#[path = "./my_pthread/my_pthread.rs"] mod my_pthread;
#[path = "./my_pthread/animation_processor.rs"] mod animation_processor;
use ncurses::*;
use std::ffi::c_void;
use libc::*;

let configuration:*mut animation_processor::config;
let mut fieldLock: i64;

// Char para explosion de figura
let mut Top:[char; 11] = {' ', ' ', ' ', ' ', ' ', ' ',' ',' ',' ',' ',' '};
let mut SecTop:[char; 11] = {' ', ' ', ' ', ' ', ' ', ' ',' ',' ',' ',' ',' '};
let mut mid:[char; 11] = {' ', ' ', ' ', ' ', ' ', ' ',' ',' ',' ',' ',' '};
let mut SecBot:[char; 11] = {' ', ' ', ' ', ' ', ' ', ' ',' ',' ',' ',' ',' '};
let mut Bot:[char; 11] = {' ', ' ', ' ', ' ', ' ', ' ',' ',' ',' ',' ',' '};

// Char para explosion de figura
let mut TopExp:[char; 10] = {' ', ' ', ' ', ' ', ' ', ' ',' ',' ',' ',' '};
let mut SecTopExp:[char; 10] = {' ', ' ', '*', '*', '*', ' ',' ',' ',' ',' '};
let mut midExp:[char; 10] = {' ', ' ', '*', '*', '*', '*','*',' ',' ',' '};
let mut SecBotExp:[char; 10] = {' ', ' ', '*', '*', '*', ' ',' ',' ',' ',' '};
let mut BotExp:[char; 10] = {' ', ' ', ' ', ' ', ' ', ' ',' ',' ',' ',' '};

// Char para explosion de figura
let mut TopExpFin:[char; 10] = {' ', ' ', ' ', '*', '*', '*',' ',' ',' ',' '};
let mut SecTopExpFin:[char; 10] = {' ', ' ', '*', '*', '*', '*','*',' ',' ',' '};
let mut midExpFin:[char; 10] = {' ', '*', '*', '*', '*', '*','*','*',' ',' '};
let mut SecBotExpFin:[char; 10] = {' ', ' ', '*', '*', '*', '*','*',' ',' ',' '};
let mut BotExpFin:[char; 10] = {' ', ' ', ' ', '*', '*', '*',' ',' ',' ',' '};


/** initialize_animation_lock
 * No recibe nada
 * No retorna nada
 * Inicializa el mutex a utilizar en la animacion
 */
fn initialize_animation_lock(){
  my_mutex::my_mutex_init(&fieldLock);

}

/** move_figure
 * Recibe los argumentos para iniciar la figura
 * No retorna nada
 * Metodo que implementa los movimientos de la ciudad
 * ademas prepara el mutex para la ciudad
 */
fn move_figure(arg: *mut c_void){
   my_mutex::my_mutex_lock(&fieldLock);
   
   // Conversion del parametro de la funcion del hilo
   // a la estructura item_info
   let mut figure = (item_info *) arg;
   
   my_mutex::my_mutex_unlock(&fieldLock);

   my_mutex::my_mutex_lock(&fieldLock);
   
   // Se recorren los monitores para encontrar al monitor que pertenece la figura descrita
   let mut temp_monitor: *mut animation_processor::monitor_info = (*mut animation_processor::monitor_info) libc::malloc(libc::sizeof(animation_processor::monitor_info));
   
   temp_monitor = configuration.monitors_list.head;

   while(temp_monitor.id != figure.monitor_id){
     
     temp_monitor = temp_monitor.next;
   }

   my_mutex::my_mutex_unlock(&fieldLock);

   my_mutex::my_mutex_lock(&fieldLock);
   
   // Asigna las posiciones iniciales de la figura
   figure.posicion_actual_x = figure.posicion_inicial_x;
   
   figure.posicion_actual_y = figure.posicion_inicial_y;
   
   my_mutex::my_mutex_unlock(&fieldLock);
   
   // Verifica si es momento de iniciar la animacion de la figura
   while(1){
     
     // Verifica si el tiempo recorrido es menor al tiempo de inicio
     // Si lo es, cede el procesador mediante un yield()
     if(time(0) < figure.tiempo_de_inicio){
       my_pthread::my_thread_yield();
     }

     // Sino inicia la animacion
     else{

       // Recorro las posiciones que tiene la figura en su configuracion
       // hasta que llegue a la posicion final
       while(figure.posicion_actual_x != figure.posicion_final_x || figure.posicion_actual_y != figure.posicion_final_y) {


            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_y <= figure.posicion_final_y){
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x-1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+2, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+3, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+4, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+5, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+6, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+7, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+8, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+9, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+10, " ");
            }

            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_x <= figure.posicion_final_x){
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x-1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x-1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x-1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x-1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x-1, " ");

            }

            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_x >= figure.posicion_final_x){
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x+11, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x+11, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x+11, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x+11, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x+11, " ");
            }

            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_y >= figure.posicion_final_y){
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x-1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+1, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+2, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+3, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+4, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+5, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+6, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+7, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+8, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+9, " ");
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+10, " ");
            }

            // Muestra la posicion actual de la figura
            ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, figure.ascii_item[0]);
            ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, figure.ascii_item[1]);
            ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, figure.ascii_item[2]);
            ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, figure.ascii_item[3]);
            ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, figure.ascii_item[4]);

            // Verifica que la figura no se le haya acabado el tiempo de ejecucion asignado
            // Muestra animacion de explosion
            if(time(0) > figure.tiempo_de_fin){
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, Top);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTop);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, mid);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBot);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, Bot);
              wrefresh(temp_monitor. canvas_window);
              usleep(800000);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, TopExp);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTopExp);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, midExp);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBotExp);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, BotExp);
              wrefresh(temp_monitor. canvas_window);
              usleep(800000);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, TopExpFin);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTopExpFin);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, midExpFin);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBotExpFin);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, BotExpFin);
              wrefresh(temp_monitor. canvas_window);
              usleep(700000);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, Top);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTop);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, mid);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBot);
              ncurses::mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, Bot);
              wrefresh(temp_monitor. canvas_window);
              break;
            }


            my_mutex::my_mutex_lock(&fieldLock);

            // Se encarga de calcular la siguiente posicion de la figura
            if(figure.posicion_actual_y < figure.posicion_final_y)
              figure.posicion_actual_y++;
            
            // Se encarga de calcular la siguiente posicion de la figura
            if(figure.posicion_actual_x < figure.posicion_final_x)
              figure.posicion_actual_x++;

            // Se encarga de calcular la siguiente posicion de la figura
            if(figure.posicion_actual_y > figure.posicion_final_y)
              figure.posicion_actual_y--;
            
            // Se encarga de calcular la siguiente posicion de la figura
            if(figure.posicion_actual_x > figure.posicion_final_x)
              figure.posicion_actual_x--;
              my_mutex::my_mutex_unlock(&fieldLock);

        // Refresca los valores que se le han asignado a la ventana y los aplica
        mvwprintw::wrefresh(temp_monitor. canvas_window);

        usleep(900000); // Shorter delay between movements
      }
      break;
    }
  }
}