use rapier2d::prelude::*;
use shared::protocol::ServerMessage;
use shared::types::VehicleState;

use crate::map::Arena;
use crate::vehicle::Vehicle;

pub struct Game {
    pub tick: u64,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub arena: Arena,
    pub player: Option<Vehicle>,
    next_id: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();

        let arena = Arena::new(&mut bodies, &mut colliders);

        let mut integration_parameters = IntegrationParameters::default();
        integration_parameters.dt = 1.0 / 60.0;

        Game {
            tick: 0,
            bodies,
            colliders,
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            integration_parameters,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            arena,
            player: None,
            next_id: 1,
        }
    }

    pub fn add_player(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let vehicle = Vehicle::spawn(
            id,
            0.0,
            -10.0,
            &mut self.bodies,
            &mut self.colliders,
            false,
        );
        self.player = Some(vehicle);
        id
    }

    pub fn remove_player(&mut self) {
        if let Some(vehicle) = self.player.take() {
            self.bodies.remove(
                vehicle.body_handle,
                &mut self.island_manager,
                &mut self.colliders,
                &mut self.impulse_joints,
                &mut self.multibody_joints,
                true,
            );
        }
    }

    pub fn set_player_input(&mut self, left: bool, right: bool) {
        if let Some(ref mut vehicle) = self.player {
            vehicle.left_thrust = left;
            vehicle.right_thrust = right;
        }
    }

    pub fn step(&mut self) {
        // Apply thrust forces
        if let Some(ref vehicle) = self.player {
            vehicle.apply_thrust(&mut self.bodies);
        }

        // Step physics
        let gravity = vector![0.0, 0.0];
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );

        self.tick += 1;
    }

    pub fn get_state_message(&self) -> ServerMessage {
        let mut vehicles: Vec<VehicleState> = Vec::new();

        if let Some(ref vehicle) = self.player {
            vehicles.push(vehicle.to_state(&self.bodies));
        }

        ServerMessage::State {
            tick: self.tick,
            vehicles,
        }
    }
}
