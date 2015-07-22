macro_rules! struct_events {
(
	keyboard: { $( $k_alias:ident : $k_sdl:ident ),* }
) => {
	use self::sdl2::event::EventPump;
	
	pub struct ImmediateEvents{
		$(pub $k_alias: Option<bool>),*
	}
	
	impl ImmediateEvents{
		pub new() -> ImmediateEvents{
			ImmediateEvents {
				$($k_alias: None),*
			}
		}
	}
	
	pub struct Events<'a>{
		pump: EventPump<'a>,
		pub now: ImmediateEvents,
		$(pub $k_alias: bool),*
	}
	
	impl<'a> Events<'a>{
		pub fn new(pump: EventPump<'a>) -> Events<'a> {
			Events{
				pump: pump,
				now: ImmediateEvents::new();
				$($k_alias: false),*
			}
		}
		
		pub fn pump(&mut self){
			self.now = ImmediateEvents::new();
			
			for event in self.pump.poll_iter(){
				use ::sdl::event::Event::*;
				use ::sdl::keyboard::Keycode::*;
				
				match event{
					KeyCode{keycode, ..} => match keycode {
						
					}
					_ => {}
					}
				}
			}
		}
}