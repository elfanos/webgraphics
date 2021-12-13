use super::super::common_funcs as cf;
use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

pub struct Color2D {
	program: WebGlProgram,
	rect_vertice_ary_len: usize,
	rect_vertice_buffer: WebGlBuffer,
	u_color: WebGlUniformLocation,
	u_opacity: WebGlUniformLocation,
	u_transform: WebGlUniformLocation,
}

impl Color2D {
	pub fn new(gl: &WebGlRenderingContext) -> Self {
		let program = cf::link_program(
			gl,
			super::super::shaders::vertex::color_2d::SHADER,
			super::super::shaders::fragment::color_2d::SHADER,
		)
		.unwrap();

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

	pub fn render(
		&self,
		gl: &WebGlRenderingContext,
		bottom: f32,
		top: f32,
		left: f32,
		right: f32,
		canvas_height: f32,
		canvas_width: f32,
	) {
		gl.use_program(Some(&self.program));
		gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
		gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
		gl.enable_vertex_attrib_array(0);

		gl.uniform4f(
			Some(&self.u_color),
			0.,  //r
			0.5, //g
			0.5, //b
			1.0, //a
		);

		gl.uniform1f(Some(&self.u_opacity), 1.);

		// nex x axis translation
		let new_x_translation = cf::get_x_translation(canvas_width, 600., 600.);

		let scaling = canvas_height / 20.;

		// nex x axis translation
		let new_y_translation = cf::get_y_translation(canvas_height,3.);


		// leader
		// should have the correct
		let translations_x = format!("test colio {:?}  {:?}", new_x_translation,new_y_translation);
		log(&translations_x);
		let x_origin_translation = 2. * new_x_translation / canvas_width - 1.; // x translation

		let y_origin_translation = 2. * new_y_translation / canvas_height - 1.;

		let translation_matrix =
			cf::translation_matrix(x_origin_translation, y_origin_translation, 0.);

		let scaling_4d_matrix = cf::scaling_matrix_4x4(
			2. * (right - left) / canvas_width,
			2. * (top - bottom) / canvas_height,
			0.,
		);

		let transform_4d_mat = cf::mult_2d_matrix_4x4(scaling_4d_matrix, translation_matrix);

		let together = format!(
			"test colio {:?}  {:?} {:?}  {:?} ",
			2. * left / canvas_width - 1.,
			2. * bottom / canvas_height - 1.,
			bottom,
			canvas_height
		);
		log(&together);

		gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_4d_mat);

		gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_ary_len / 2) as i32);
	}
}
