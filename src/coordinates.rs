
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
