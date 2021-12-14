use super::super::coordinates as Coordinates;
use super::super::matrix_equations as Matrix;
use super::super::webgl_compiler as GLCompiler;
use crate::objects::game_box::{GameBox as GameBox2d};
use crate::objects::matrix_translation::{MatrixtTranslation};
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

pub struct GameBox {
	program: WebGlProgram,
	rect_vertice_ary_len: usize,
	rect_vertice_buffer: WebGlBuffer,
	u_color: WebGlUniformLocation,
	u_opacity: WebGlUniformLocation,
	u_transform: WebGlUniformLocation,
}

impl GameBox {
	pub fn new(gl: &WebGlRenderingContext) -> Self {
		let program = GLCompiler::link_program(
			gl,
			super::super::shaders::vertex::game_box::SHADER,
			super::super::shaders::fragment::game_box::SHADER,
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
		canvas_height: f32,
		canvas_width: f32,
		box_2d: &GameBox2d,
		xyt:MatrixtTranslation,

	) {

		let translation_matrix =
			Matrix::translation_matrix_4x4(xyt.xt, xyt.yt, 0.);

		//
		let scaling_4d_matrix = Matrix::scaling_matrix_4x4(
			2. * (box_2d.position.right - box_2d.position.left) / canvas_width,
			2. * (box_2d.position.top - box_2d.position.bottom) / canvas_height,
			0.,
		);

		let transform_4d_mat = Matrix::mult_matrix_4x4(scaling_4d_matrix, translation_matrix);

		/** add to gl compiler function */
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
		gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_4d_mat);
		gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_ary_len / 2) as i32);
	}
}

// pub fn get_matrix_transformation() -> [f32;16] {

// }
