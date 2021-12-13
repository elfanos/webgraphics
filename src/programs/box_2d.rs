use super::super::common_funcs as cf;
use rand::Rng;
use super::super::objects::game_box::GameBox;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Box2d {
	program: WebGlProgram,
	rect_vertice_ary_len: usize,
	rect_vertice_buffer: WebGlBuffer,
	u_color: WebGlUniformLocation,
	u_opacity: WebGlUniformLocation,
	u_transform: WebGlUniformLocation,
}

impl Box2d {
	pub fn new(gl: &WebGlRenderingContext) -> Self {
		let program = cf::link_program(
			gl,
			super::super::shaders::vertex::box_2d::SHADER,
			super::super::shaders::fragment::box_2d::SHADER,
		)
		.unwrap();

		// creating a rec shape
		let vertices_rect: [f32; 12] = [
			0., 1., // x, y
			0., 0., // x, y
			1., 1., // x, y
			1., 1., // x, y
			0., 0., //x, y
			1., 0., //x, y
		];

		let memory_buffer = wasm_bindgen::memory()
			.dyn_into::<WebAssembly::Memory>()
			.unwrap()
			.buffer();

		let vertices_location = vertices_rect.as_ptr() as u32 / 4;

		let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
			vertices_location,
			vertices_location + vertices_rect.len() as u32,
		);
		let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();

		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
		gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
		Self {
			u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
			u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
			u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
			rect_vertice_ary_len: vertices_rect.len(),
			rect_vertice_buffer: buffer_rect,
			program,
		}
	}
	pub fn render(&self, gl: &WebGlRenderingContext, game_box: &GameBox) {
		// getting props from the gamebox
		let left = game_box.get_position().get_left();
		let right = game_box.get_position().get_right();
		let bottom = game_box.get_position().get_bottom();
		let top = game_box.get_position().get_top();
		let width = game_box.get_width();
		let height = game_box.get_height();

		

		let together = format!("test colio {:?} {:?} {:?} {:?} {:?} {:?}", left, right,bottom,top,width, height);
		log(&together);
		let mut rng = rand::thread_rng();
		gl.use_program(Some(&self.program));
		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
		gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
		gl.enable_vertex_attrib_array(0);
		gl.uniform4f(
			Some(&self.u_color),
			0.,  //r
			1., //g
			0.5, //b
			1.0, //a
		);

		gl.uniform1f(Some(&self.u_opacity), 1.);
		let translation_matrix =
			cf::translation_matrix(2. * left / width - 1., 2. * bottom / height - 1., 0.);

		let scale_matrix = cf::scaling_matrix(
			2. * (right - left) / width,
			2. * (top - bottom) / height,
			0.,
		);

		let transform_mat = cf::mult_matrix_4(scale_matrix, translation_matrix);

		gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

		gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_ary_len / 2) as i32);
	}
}