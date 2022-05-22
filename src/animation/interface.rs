use ncurses::*;
use libc::*;
mod "../my_pthread/animation_processsor.rs"

let mut configuration: *mut config;
/** create_interface
 * crea la ventana para mostrar la ciudad
 * le asigna el tamaño a esta
 */
void create_interface(){

  // Inicializacion de ncurses (necesario)
  let mut x: i64 = 0;
  let mut y: i64 = 0;
  
  initscr();
  
  noecho();
  
  curs_set(false);

  // Puntero temporal que nos ayudara a recorrer la lista de monitores
  let mut temp_monitor: *mut monitor_info = (*mut monitor_info) malloc(sizeof(monitor_info));
  temp_monitor = configuration.monitors_list.head;

  // Se recorre la lista de monitores en config
	while(temp_monitor != NULL){

      // Se crea un nuevo window por cada monitor y se guarda en la estructura
      temp_monitor.curses_window = newwin(temp_monitor.height_curses_size,temp_monitor.width_curses_size, y, x);
      
      // Se posicionan en la terminal segun el ancho de cada uno
      x += temp_monitor.width_curses_size;
      
      //y+=temp_monitor.height_curses_size;
      
      // Funcion de ncurses para pintar los bordes del window
      box(temp_monitor.curses_window, 0, 0);
      
      // Es necesario para mostrar los cambios hechos en window
      wrefresh(temp_monitor.curses_window);
		  
      // Se mueve al siguiente
      temp_monitor = temp_monitor.next;
	}
}