use rapier2d::prelude::*;

pub const ARENA_HALF_WIDTH: f32 = 80.0;
pub const ARENA_HALF_HEIGHT: f32 = 60.0;
const WALL_THICKNESS: f32 = 1.0;

pub struct Arena {
    pub half_width: f32,
    pub half_height: f32,
}

impl Arena {
    pub fn new(bodies: &mut RigidBodySet, colliders: &mut ColliderSet) -> Self {
        let hw = ARENA_HALF_WIDTH;
        let hh = ARENA_HALF_HEIGHT;
        let wt = WALL_THICKNESS;

        // Walls: top, bottom, left, right
        let walls = [
            (0.0, hh + wt, hw + wt, wt),   // top
            (0.0, -hh - wt, hw + wt, wt),  // bottom
            (-hw - wt, 0.0, wt, hh + wt),  // left
            (hw + wt, 0.0, wt, hh + wt),   // right
        ];

        for (x, y, half_w, half_h) in walls {
            let body = RigidBodyBuilder::fixed()
                .translation(vector![x, y])
                .build();
            let handle = bodies.insert(body);
            let collider = ColliderBuilder::cuboid(half_w, half_h)
                .restitution(0.8)
                .build();
            colliders.insert_with_parent(collider, handle, bodies);
        }

        Arena {
            half_width: hw,
            half_height: hh,
        }
    }
}
