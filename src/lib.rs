use framebuffer::Framebuffer;
use framebuffer::KdMode;
use framebuffer::FramebufferError;
use std::ops::Add;
use std::ops::AddAssign;

/// Represents a pixel on the screen
pub struct Pixel {
	index: usize,
}

impl Pixel {
	/// Sets the `Pixel` to the given RGB value in the given buffer
	pub fn set_rgb(&self, buffer: &mut [u8], r: u8, g: u8, b: u8) {
		buffer[self.index]=b;
		buffer[self.index+1]=g;
		buffer[self.index+2]=r;
	}
	/// Gets the current color of the `Pixel`
	pub fn get_rgb(&self, buffer: &[u8]) -> (u8,u8,u8) {
		(buffer[self.index+2], buffer[self.index+1], buffer[self.index])
	}
}

/// Represents a point
#[derive(Clone,Copy)]
pub struct Point {
	pub x: usize,
	pub y: usize,
}

impl Point{
	/// Creates a new `Point` with the given x, y coordinates
	pub fn new(x: usize, y: usize) -> Self {
		Point{x,y}
	}
}

impl Add<(usize,usize)> for Point {
	type Output = Self;

	/// Adds a `(u8,u8)` to the Point
	///
	/// #Examples
	///
	/// ```
	/// use framebuffer_manager::Point;
	///
	/// let p = Point::new(5,6);
	/// let new_p = p + (1,2);
	/// assert_eq!(new_p.x, 6);
	/// assert_eq!(new_p.y, 8);
	/// ```
	fn add(self, offset: (usize, usize)) -> Self {
		Self {
			x: self.x + offset.0,
			y: self.y + offset.1,
		}
	}
}

impl AddAssign<(usize,usize)> for Point {
	fn add_assign(&mut self, offset: (usize, usize)){
		self.x += offset.0;
		self.y += offset.1;
	}
}

impl From<(usize,usize)> for Point {
	fn from(p: (usize, usize)) -> Self {
		Point {x: p.0, y: p.1}
	}
}

pub struct Rectangle {
	pub location: Point,
	pub height: usize,
	pub width: usize,
	pub pixels: Vec<Vec<Pixel>>,
}

impl Rectangle {
	/// Creates a new `Rectangle` from the given dimensions and assigns the `Pixel`s their proper indicies based on the given `Framebuffer`
	fn from_dimensions(loc: &Point, height: usize, width: usize, fb : &Framebuffer) -> Self {
		let line_length = fb.fix_screen_info.line_length as usize;
		let bytespp = (fb.var_screen_info.bits_per_pixel / 8) as usize;
		let mut rows = Vec::new();
		for i in 0..height {
			let mut pixel_line = Vec::new();
			for k in 0..width {
				let index = ((i + loc.y) * line_length + (k + loc.x) * bytespp) as usize;
				pixel_line.push(Pixel{index});
			}
			rows.push(pixel_line);
		}
		Rectangle {
			location: *loc,
			height,
			width,
			pixels: rows,
		}
	}
	/// Fills a `Rectangle` with a given color
	fn fill(&self, buffer: &mut [u8], rgb: (u8,u8,u8)) {
		for row in self.pixels.iter() {
			for p in row.iter() {
				p.set_rgb(buffer, rgb.0, rgb.1, rgb.2);
			}
		}
	
	}
}

/// Represents a border around a `Window`
pub struct Border {
	pub top: Rectangle,
	pub bot: Rectangle,
	pub left: Rectangle,
	pub right: Rectangle,
}

/// Represents a portion of the screen
pub struct Window {
	pub border: Option<Border>,
	pub width: usize,
	pub height: usize,
	pub main_context: Rectangle,
}

impl Window {
	/// Fills a `Window`'s `main_context` with the given color
	fn fill(&self, buffer: &mut [u8], rgb: (u8,u8,u8)) {
		self.main_context.fill(buffer,rgb);
	}
	/// Fills a `Window`'s `border` with the given color
	fn fill_border(&self, buffer: &mut [u8], rgb: (u8,u8,u8)) {
		match &self.border {
			Some(br) => {
				br.top.fill(buffer, rgb);
				br.left.fill(buffer, rgb);
				br.right.fill(buffer, rgb);
				br.bot.fill(buffer, rgb);
			},
			_ => {}
		}
	}
}

/// A template to create a `Window`
pub struct WindowTemplate {
	pub id: usize,
	pub location: Point,
	pub width: usize,
	pub height: usize,
	pub border_thickness: usize,
}

/// A container to manage the framebuffer. Abstracts away from the buffer that represents the screen
pub struct FBmanager {
	pub framebuffer: Framebuffer,
	pub buffer: Vec<u8>,
	pub windows: Vec<Window>,
}

impl FBmanager {
	/// Creates a new `FBmanager` using the given template
	pub fn new(template: &[WindowTemplate]) -> Self {
		let framebuffer = Framebuffer::new("/dev/fb0").unwrap();
		let height = framebuffer.var_screen_info.yres;
		let line_length = framebuffer.fix_screen_info.line_length;
		let buffer = vec![0u8; (line_length*height) as usize];
		let mut window_holder = Vec::new();
		for t in template.iter() {
			//create border
			let mut border = None;
			let mut start_location = t.location;
			let mut context_height = t.height;
			let mut context_width = t.width;
			if t.border_thickness > 0 {
				//create top
				let border_height = t.border_thickness;
				let border_width = t.width;
				let top = Rectangle::from_dimensions(&t.location,border_height, border_width, &framebuffer);
				//create bottom
				let loc = t.location + (0, t.height - t.border_thickness);
				let bot = Rectangle::from_dimensions(&loc, border_height, border_width, &framebuffer);
				//create right
				let loc = t.location + (t.width - t.border_thickness, t.border_thickness); 
				let border_height = t.height - 2*t.border_thickness;
				let border_width = t.border_thickness;
				let right = Rectangle::from_dimensions(&loc, border_height, border_width, &framebuffer);
				//create left
				let loc = t.location + (0, t.border_thickness);
				let left = Rectangle::from_dimensions(&loc, border_height, border_width, &framebuffer);
				border = Some(Border {
					top,
					bot,
					left,
					right,
				});
				
				start_location += (t.border_thickness, t.border_thickness);
				context_height -= 2*t.border_thickness;
				context_width -= 2*t.border_thickness;
			}
			//create main_context
			let main_context = Rectangle::from_dimensions(&start_location, context_height, context_width, &framebuffer); 
			let window = Window {
				border,
				width: t.width,
				height: t.height,
				main_context,
			};
			window_holder.push(window);
		}
		FBmanager {
			framebuffer,
			buffer,
			windows: window_holder,
		}
	}
	/// Enables Framebuffer graphics. *Must be enabled to draw to the screen*
	/// **Must call** `disable_graphics()` **before the process exits**
	pub fn enable_graphics() -> Result<i32, FramebufferError> {
		Framebuffer::set_kd_mode(KdMode::Graphics)
	}
	/// Disables Framebuffer graphics. Call to use traditional means to print to the screen.
	pub fn disable_graphics() -> Result<i32, FramebufferError> {
		Framebuffer::set_kd_mode(KdMode::Text)
	}
	/// Fills the `Window` with the given `id` to the given color
	pub fn fill(&mut self, id: usize, rgb: (u8,u8,u8)) {
		self.windows[id].fill(&mut self.buffer, rgb);
	}
	/// Fills the `Window` with the given `id`'s border to the given color
	pub fn fill_border(&mut self, id: usize, rgb: (u8,u8,u8)) {
		self.windows[id].fill_border(&mut self.buffer, rgb);
	}
	/// Draws the `FBmanager`'s internal state to the screen. Remeber to `enable_graphics()` before this
	pub fn draw(&mut self) {
		self.framebuffer.write_frame( &self.buffer);
	}
}

//tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn single_square() {
		let square = WindowTemplate {
			id: 0,
			location: Point::new(0,0),
			width: 1000,
			height: 1000,
			border_thickness: 0,
		};
		let mut fm = FBmanager::new(&[square]);
		let mut win = &mut fm.windows[0];
		fm.windows[0].main_context.pixels[0][0].set_rgb(&mut fm.buffer,255,0,0);
		fm.fill(0,(255,0,0));
		let step_size = (fm.framebuffer.var_screen_info.bits_per_pixel / 8) as usize;
		let line_length: usize = fm.framebuffer.fix_screen_info.line_length as usize;
		println!("{:?}", fm.buffer);
		for i in 0..1000 {
			for q in 0..1000 {
				let index: usize = i*line_length + q*step_size;
				assert_eq!(fm.buffer[index], 0);
				assert_eq!(fm.buffer[index+1], 0);
				assert_eq!(fm.buffer[index+2], 255);
			}
		}
	}
}
