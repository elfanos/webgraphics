use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

pub fn link_program(
	gl: &WebGlRenderingContext,
	vert_source: &str,
	frag_source: &str,
) -> Result<WebGlProgram, String> {
	let program = gl
		.create_program()
		.ok_or_else(|| String::from("Error creating program"))?;

	let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();

	let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

	gl.attach_shader(&program, &vert_shader);
	gl.attach_shader(&program, &frag_shader);
	gl.link_program(&program);

	if gl
		.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
		.as_bool()
		.unwrap_or(false)
	{
		Ok(program)
	} else {
		Err(gl
			.get_program_info_log(&program)
			.unwrap_or_else(|| String::from("Unknown error creating program object")))
	}
}

fn compile_shader(
	gl: &WebGlRenderingContext,
	shader_type: u32,
	source: &str,
) -> Result<WebGlShader, String> {
	let shader = gl
		.create_shader(shader_type)
		.ok_or_else(|| String::from("Error creating shader"))?;
	gl.shader_source(&shader, source);
	gl.compile_shader(&shader);

	if gl
		.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
		.as_bool()
		.unwrap_or(false)
	{
		Ok(shader)
	} else {
		Err(gl
			.get_shader_info_log(&shader)
			.unwrap_or_else(|| String::from("Unable to get shader info log")))
	}
}

/**
 * T(x,y) = [x,y, 1] form a new vector that represent the translation in the webgl graphic
 * use row major form isntead of column major form
 */
pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];
	return_var[0] = 1.;
	return_var[5] = 1.;
	return_var[10] = 1.;
	return_var[15] = 1.;

	return_var[12] = tx;
	return_var[13] = ty;
	return_var[14] = tz;
	return_var
}

/**
 * T(x,y) = [x,y, 1] form a new vector that represent the translation in the webgl graphic
 * link -> https://www.alanzucconi.com/2016/02/10/tranfsormation-matrix/
 */
pub fn scalar_matrixt_translation_4x4(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];
	return_var[0] = 1.;
	return_var[3] = tx;

	return_var[5] = 1.;
	return_var[7] = ty;

	return_var[10] = 1.;
	return_var[11] = tz;
	return_var[15] = 1.;
	return_var
}

/**
 * T(x,y) = [x,y, 1] form a new vector that represent the translation in the webgl graphic
 * link -> https://www.alanzucconi.com/2016/02/10/tranfsormation-matrix/
 */
pub fn scalar_2d_matrixt_translation(tx: f32, ty: f32) -> [f32; 9] {
	let mut return_var = [0.; 9];
	return_var[0] = 1.;
	return_var[2] = tx;
	return_var[4] = 1.;
	return_var[5] = ty;
	return_var[8] = 1.;
	return_var
}

/**
 * T(w,h) = [w,h,y] create scaling for the vertices by multiply with a diaghonal matrix containing width and heigh as properties
 * link -> https://www.alanzucconi.com/2016/02/10/tranfsormation-matrix/
 */
pub fn scaling_2d_matrix(w: f32, h: f32) -> [f32; 9] {
	let mut return_var = [0.; 9];
	return_var[0] = w;
	return_var[4] = h;
	return_var[8] = 1.;
	return_var
}

/**
 * T(w,h) = [w,h,y] create scaling for the vertices by multiply with a diaghonal matrix containing width and heigh as properties
 * link -> https://www.alanzucconi.com/2016/02/10/tranfsormation-matrix/
 */
pub fn scaling_matrix_4x4(w: f32, h: f32, depth: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];
	return_var[0] = w;
	return_var[5] = h;
	return_var[10] = depth;
	return_var[15] = 1.;

	return_var
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
	let mut return_var = [0.; 16];
	return_var[0] = sx;
	return_var[5] = sy;
	return_var[10] = sz;
	return_var[15] = 1.;
	return_var
}

pub fn mult_2d_matrix_3x3(a: [f32; 9], b: [f32; 9]) -> [f32; 9] {
	let mut return_var = [0.; 9];
	return_var[0] = a[0] * b[0] + a[1] * b[3] + a[2] * b[6];
	return_var[1] = a[0] * b[1] + a[1] * b[4] + a[2] * b[7];
	return_var[2] = a[0] * b[2] + a[1] * b[5] + a[2] * b[8];

	return_var[3] = a[3] * b[0] + a[4] * b[3] + a[5] * b[6];
	return_var[4] = a[3] * b[1] + a[4] * b[4] + a[5] * b[7];
	return_var[5] = a[3] * b[2] + a[4] * b[5] + a[5] * b[8];

	return_var[6] = a[6] * b[0] + a[7] * b[3] + a[8] * b[6];
	return_var[7] = a[6] * b[1] + a[7] * b[4] + a[8] * b[7];
	return_var[8] = a[6] * b[2] * a[7] * b[5] + a[8] * b[8];
	return_var
}

pub fn mult_2d_matrix_4x4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
	let mut return_var = [0.; 16];
	return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
	return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
	return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
	return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

	return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
	return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
	return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
	return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

	return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
	return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
	return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
	return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

	return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
	return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
	return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
	return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

	return_var
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
	let mut return_var = [0.; 16];

	return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
	return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
	return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
	return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

	return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
	return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
	return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
	return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

	return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
	return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
	return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
	return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

	return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
	return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
	return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
	return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

	return_var
}

// leader
// should have the correct translation
// translation calculation
// given range 0 < x < width -> where width is canvas width
// return a correct translation on x-axis
// if width is greater than container_width translate out element

/**
 *	container-width' the canvas width or plane width
 * 	x_range is the most right position on the plane
 *  x_position each box x position based on the viewport
 */
pub fn get_x_translation(container_width: f32, x_range: f32, x_position: f32) -> f32 {
	let x_range_position = container_width * 0.9; // should be procentage instead
	let current_x_position = x_range - x_position; // calculate the x position of the generated viewport
	let new_x_position = x_range_position - current_x_position; // gives the translation index of x

	new_x_position
}

/**
 *	container-width' the canvas width or plane width
 * 	y_range is the most top position on the plane
 *  x_position each box x position based on the viewport
 */
pub fn get_y_translation(container_height: f32, y_position: f32) -> f32 {
	let y_range_position = container_height * 0.9; // should be procentage instead
	let new_x_position = y_range_position - y_position * ( y_range_position / 20.); // gives the translation index of x
	new_x_position
}
