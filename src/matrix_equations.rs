/**
 * T(x,y) = [x,y, 1] form a new vector that represent the translation in the webgl graphic
 * use row major form isntead of column major form
 * open gl seems to use row major form
 */
pub fn translation_matrix_4x4(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
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

/**
 * Multiply 2 4x4 matrix
 */
pub fn mult_matrix_4x4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
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