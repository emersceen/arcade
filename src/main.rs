mod events;
extern crate sdl2 as sdl;

#[macro_use] mod events;

struct_events!(
    keyboard: {
        key_escape: Escape
    }
);

use sdl::pixels::Color;

fn main() {
    let mut sdl = sdl::init().video()
              .build().unwrap();
    let window = sdl.window("arcade",800,600)
                 .position_centered().opengl()
                 .build().unwrap();
    let mut rndrer = window.renderer()
                     .accelerated()
                     .build().unwrap();
    
    let mut events = events::Events::new(sdl.event_pump());
    'game_loop: loop{
        events.pump();
        
        if true {
            break 'game_loop;
        }
        rndrer.set_draw_color(Color::RGB(117,25,90));
        rndrer.clear();
        rndrer.present();
    }         
}
