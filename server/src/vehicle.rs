use rapier2d::prelude::*;

// Circle radii
pub const ENGINE_RADIUS: f32 = 0.5;
pub const COCKPIT_RADIUS: f32 = 0.4;

// Layout offsets from body origin
pub const ENGINE_OFFSET_X: f32 = 1.5;
pub const ENGINE_OFFSET_Y: f32 = 0.8;
pub const COCKPIT_OFFSET_Y: f32 = -0.8;

// Physics tuning
const BOTH_FORWARD_FORCE: f32 = 21.0;
const SINGLE_FORWARD_FORCE: f32 = 5.0;
const TURN_TORQUE: f32 = 10.0;
const ENGINE_DENSITY: f32 = 2.0;
const COCKPIT_DENSITY: f32 = 1.5;
const RESTITUTION: f32 = 0.6;

pub struct Vehicle {
    pub id: u32,
    pub body_handle: RigidBodyHandle,
    pub left_thrust: bool,
    pub right_thrust: bool,
    pub is_ghost: bool,
}

impl Vehicle {
    pub fn spawn(
        id: u32,
        x: f32,
        y: f32,
        bodies: &mut RigidBodySet,
        colliders: &mut ColliderSet,
        is_ghost: bool,
    ) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .linear_damping(26.0)
            .angular_damping(10.0)
            .build();
        let body_handle = bodies.insert(rigid_body);

        let left_engine = ColliderBuilder::ball(ENGINE_RADIUS)
            .density(ENGINE_DENSITY)
            .friction(0.1)
            .restitution(RESTITUTION)
            .translation(vector![-ENGINE_OFFSET_X, ENGINE_OFFSET_Y])
            .build();
        colliders.insert_with_parent(left_engine, body_handle, bodies);

        let right_engine = ColliderBuilder::ball(ENGINE_RADIUS)
            .density(ENGINE_DENSITY)
            .friction(0.1)
            .restitution(RESTITUTION)
            .translation(vector![ENGINE_OFFSET_X, ENGINE_OFFSET_Y])
            .build();
        colliders.insert_with_parent(right_engine, body_handle, bodies);

        let cockpit = ColliderBuilder::ball(COCKPIT_RADIUS)
            .density(COCKPIT_DENSITY)
            .friction(0.1)
            .restitution(RESTITUTION)
            .translation(vector![0.0, COCKPIT_OFFSET_Y])
            .build();
        colliders.insert_with_parent(cockpit, body_handle, bodies);

        Vehicle {
            id,
            body_handle,
            left_thrust: false,
            right_thrust: false,
            is_ghost,
        }
    }

    pub fn apply_thrust(&self, bodies: &mut RigidBodySet) {
        let body = &bodies[self.body_handle];
        let angle = body.rotation().angle();

        let forward = vector![(-angle).sin(), angle.cos()];

        let left = self.left_thrust;
        let right = self.right_thrust;
        let body = &mut bodies[self.body_handle];

        if left && right {
            body.add_force(forward * BOTH_FORWARD_FORCE * 2.0, true);
        } else if left || right {
            body.add_force(forward * SINGLE_FORWARD_FORCE, true);
        }

        if left && !right {
            body.add_torque(-TURN_TORQUE, true);
        } else if right && !left {
            body.add_torque(TURN_TORQUE, true);
        }
    }

    pub fn to_state(&self, bodies: &RigidBodySet) -> shared::types::VehicleState {
        let body = &bodies[self.body_handle];
        let pos = body.translation();
        let vel = body.linvel();
        shared::types::VehicleState {
            id: self.id,
            x: pos.x,
            y: pos.y,
            angle: body.rotation().angle(),
            vx: vel.x,
            vy: vel.y,
            angular_vel: body.angvel(),
            left_thrust: self.left_thrust,
            right_thrust: self.right_thrust,
            is_ghost: self.is_ghost,
        }
    }
}
