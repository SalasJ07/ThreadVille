mod "../my_pthread/my_mutex.rs"
use crate::my_pthread::my_mutex::*;
use ncurses::*;
use std::ffi::c_void;
use libc::*;

let configuration:*mut config;
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
  my_mutex_init(&fieldLock);

}

/** move_figure
 * Recibe los argumentos para iniciar la figura
 * No retorna nada
 * Metodo que implementa los movimientos de la ciudad
 * ademas prepara el mutex para la ciudad
 */
fn move_figure(arg: *mut c_void){
    my_mutex_lock(&fieldLock);
   
   // Conversion del parametro de la funcion del hilo
   // a la estructura item_info
   item_info *figure = (item_info *) arg;
   
   my_mutex_unlock(&fieldLock);

   my_mutex_lock(&fieldLock);
   
   // Se recorren los monitores para encontrar al monitor que pertenece la figura descrita
   let mut temp_monitor: *mut monitor_info = (*mut monitor_info) malloc(sizeof(monitor_info));
   
   temp_monitor = configuration.monitors_list.head;

   while(temp_monitor.id != figure.monitor_id){
     
     temp_monitor = temp_monitor.next;
   }

   my_mutex_unlock(&fieldLock);

   my_mutex_lock(&fieldLock);
   
   // Asigna las posiciones iniciales de la figura
   figure.posicion_actual_x = figure.posicion_inicial_x;
   
   figure.posicion_actual_y = figure.posicion_inicial_y;
   
   my_mutex_unlock(&fieldLock);
   
   // Verifica si es momento de iniciar la animacion de la figura
   while(1){
     
     // Verifica si el tiempo recorrido es menor al tiempo de inicio
     // Si lo es, cede el procesador mediante un yield()
     if(time(0) < figure.tiempo_de_inicio){
      
       my_thread_yield();
     }

     // Sino inicia la animacion
     else{

       // Recorro las posiciones que tiene la figura en su configuracion
       // hasta que llegue a la posicion final
       while(figure.posicion_actual_x != figure.posicion_final_x || figure.posicion_actual_y != figure.posicion_final_y) {


            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_y <= figure.posicion_final_y){
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x-1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+2, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+3, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+4, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+5, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+6, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+7, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+8, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+9, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-3, figure.posicion_actual_x+10, " ");
            }

            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_x <= figure.posicion_final_x){
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x-1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x-1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x-1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x-1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x-1, " ");

            }

            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_x >= figure.posicion_final_x){
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x+11, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x+11, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x+11, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x+11, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x+11, " ");
            }

            // Se encargan de limpiar la pantalla de la posicion anterior
            if(figure.posicion_actual_y >= figure.posicion_final_y){
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x-1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+1, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+2, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+3, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+4, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+5, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+6, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+7, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+8, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+9, " ");
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+3, figure.posicion_actual_x+10, " ");
            }

            // Muestra la posicion actual de la figura
            mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, figure.ascii_item[0]);
            mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, figure.ascii_item[1]);
            mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, figure.ascii_item[2]);
            mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, figure.ascii_item[3]);
            mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, figure.ascii_item[4]);

            // Verifica que la figura no se le haya acabado el tiempo de ejecucion asignado
            // Muestra animacion de explosion
            if(time(0) > figure.tiempo_de_fin){
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, Top);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTop);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, mid);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBot);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, Bot);
              wrefresh(temp_monitor. canvas_window);
              usleep(800000);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, TopExp);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTopExp);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, midExp);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBotExp);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, BotExp);
              wrefresh(temp_monitor. canvas_window);
              usleep(800000);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, TopExpFin);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTopExpFin);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, midExpFin);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBotExpFin);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, BotExpFin);
              wrefresh(temp_monitor. canvas_window);
              usleep(700000);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-2, figure.posicion_actual_x, Top);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y-1, figure.posicion_actual_x, SecTop);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y, figure.posicion_actual_x, mid);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+1, figure.posicion_actual_x, SecBot);
              mvwprintw(temp_monitor.canvas_window,figure.posicion_actual_y+2, figure.posicion_actual_x, Bot);
              wrefresh(temp_monitor. canvas_window);
              break;
            }


            my_mutex_lock(&fieldLock);

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
            my_mutex_unlock(&fieldLock);

        // Refresca los valores que se le han asignado a la ventana y los aplica
        wrefresh(temp_monitor. canvas_window);

        usleep(900000); // Shorter delay between movements
      }
      break;
    }
  }
}