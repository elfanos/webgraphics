use crate::objects::{GameBox, MatrixtTranslation};

/**
 *	container-width' the canvas width or plane width
 * 	x_range is the most right position on the plane
 *  x_position each box x position based on the viewport
 */
pub fn get_x_position(container_width: f32, x_range: f32, x_position: f32) -> f32 {
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
pub fn get_y_position(container_height: f32, y_position: f32) -> f32 {
    let y_range_position = container_height * 0.9; // should be procentage instead
    let new_x_position = y_range_position - y_position * (y_range_position / 20.); // gives the translation index of x
    new_x_position
}

/**
 * Get vector point in the plane
 * [xt,yt]
 */
pub fn get_vector_point(
    width: f32,
    height: f32,
    x_translation: f32,
    y_translation: f32,
    leader: f32,
) -> MatrixtTranslation {
    MatrixtTranslation {
        xt: get_x_position(width, leader, x_translation),
        yt: get_y_position(height, y_translation),
    }
}

/**
 * Using distance/time = velocity then to calculate unit per frame velocity/(fps => 1000 / 60 or 1000 / 30) = units per frame based on the speed
 *
 */
pub fn get_next_distance(distance: f32, time: f32, fps: f32) -> f32 {
    let units_per_frame = (distance / time) * fps;
    units_per_frame
}
pub fn get_race_leader(boxes: &Vec<GameBox>) -> f32 {
    let mut leader = 0.;
    for x in boxes {
        if leader < x.horse_position.distance_covered {
            leader = x.horse_position.distance_covered;
        }
    }
    leader
}

pub fn scale_vec_x_point(x: f32, width: f32) -> f32 {
    2. * x / width - 1.
}

pub fn scale_vec_y_point(y: f32, height: f32) -> f32 {
    2. * y / height - 1.
}

pub fn get_vector_point_in_plane(
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    leader: f32,
) -> MatrixtTranslation {
    MatrixtTranslation {
        xt: scale_vec_x_point(get_x_position(width, leader,x), width),
        yt: scale_vec_y_point(get_y_position(height, y), height),
    }
}


// pub fn get_y_translation(height: f32, y_translation: f32) -> f32 {
//     let new_y_translation = Coordinates::get_y_position(height, y_translation);
//     2. * new_y_translation / height - 1.
// }

// pub fn get_x_position(width: f32, x_translation: f32, pivot: f32) -> f32 {
//     let new_x_translation = Coordinates::get_x_position(width, pivot, x_translation);
//     2. * new_x_translation / width - 1.
// }