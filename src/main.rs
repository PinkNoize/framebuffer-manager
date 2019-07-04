use framebuffer_manager::*;
use std::io;

fn main() {
	let mut template = Vec::new();
	let background = WindowTemplate {
		id: 0,
		location: Point::new(0,0),
		width: 1280,
		height: 1024,
		border_thickness: 5,
	};
	template.push(background);
	let title = WindowTemplate {
		id: 1,
		location: Point::new(0,0),
		width: 854,
		height: 512,
		border_thickness: 0,
	};
	template.push(title);
	let picture = WindowTemplate {
		id: 2,
		location: Point::new(854,0),
		width: 426,
		height: 512,
		border_thickness: 5,
	};
	template.push(picture);
	let status = WindowTemplate {
		id: 3,
		location: Point::new(0,512),
		width: 1280,
		height: 410,
		border_thickness: 0,
	};
	template.push(status);
	let bargraph =  WindowTemplate {
		id: 4,
		location: Point::new(0,922),
		width: 1280,
		height: 102,
		border_thickness: 5,
	};
	template.push(bargraph);
	let mut fm = FBmanager::new(&template);
	fm.fill(1,(0,255,0));
	fm.fill(2,(0,0,255));
	fm.fill(3,(255,255,0));
	fm.fill(4,(0,255,255));
	fm.fill_border(0,(255,0,0));
	framebuffer_manager::FBmanager::enable_graphics().unwrap();
	fm.draw();
	io::stdin().read_line(&mut String::new()).unwrap_or_default();
	framebuffer_manager::FBmanager::disable_graphics().unwrap();
}
