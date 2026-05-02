use glam::Vec2;

pub struct Ray {
    pub origin: Vec2,     // O
    pub direction: Vec2   // D
}

pub struct Wall {
    pub start: Vec2, // A
    pub end: Vec2,   // B
    pub absorption: f32,
}

pub fn check_intersection(ray: &Ray, wall: &Wall) -> Option<Ray> {
    let from_ray_to_wall_start = wall.start - ray.origin; //Vec2::new(wall.start.x - ray.origin.x, wall.start.y - ray.origin.y); // E
    let wall_vec = wall.end - wall.start;//Vec2::new(wall.end.x - wall.start.x, wall.end.y - wall.start.y); // V

    let det = ray.direction.perp_dot(wall_vec);
    if det.abs() < 1e-6 {
        return None;
    }
    let ray_t = (from_ray_to_wall_start.perp_dot(wall_vec))/det;        // how far the ray travels
    let wall_u = (from_ray_to_wall_start.perp_dot(ray.direction))/det;  // where on the wall it hits

     if ray_t < 0.0 {
        return None;
    } else if wall_u < 0.0 || wall_u > 1.0 {
        return None;
    }
    let hit_point = ray.origin + (ray_t * wall_u);
    let wall_u_normalized = Vec2::new(-wall_vec.y, wall_vec.x).normalize();
    let bounced_ray = ray.direction.reflect(wall_u_normalized);


    Some(Ray {
        origin: hit_point,
        direction: bounced_ray,
    })
}

pub fn check_mic_intersection(
    start: Vec2,
    end: Vec2,
    mic_center: Vec2,
    mic_radius: f32
) -> bool {
    let segment = end - start;
    let segment_length_sq = segment.length_squared();

    if segment_length_sq == 0.0 {       // if ray didnt move at all
        return start.distance(mic_center) <= mic_radius;
    }
    let to_mic = mic_center - start;                     // Vector pointing from start of the ray to the microphone
    let t = to_mic.dot(segment) / segment_length_sq;      // Vector Projection
    let clamped_t = t.clamp(0.0, 1.0);          // Only the segment not the inf line
    let closest_point = start + (segment * clamped_t);  // Coordinates of the closes point on the segment
    closest_point.distance(mic_center) <= mic_radius          // Is the closes point inside the circle?

}