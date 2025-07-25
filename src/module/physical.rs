/*
* Copyright (c) 2025 luxreduxdelux
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use std::sync::{Arc, Mutex};

use crate::module::{
    general::{Box2, Color, Vec2},
    video::Frame,
};

//================================================================

use rapier2d::{control::KinematicCharacterController, prelude::*};
use rune::{Any, Module, Ref};

//================================================================

/// A handle to a 2-D physical simulation.
#[derive(Any)]
#[rune(item = ::physical)]
struct Physical2D {
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    debug_render: DebugRenderPipeline,
    collision_handler: CollisionHandler,
}

impl Physical2D {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::tick)?;
        module.function_meta(Self::draw_debug)?;

        Ok(())
    }

    //================================================================

    /// Create a new 2-D physical simulation instance.
    #[rune::function(path = Self::new)]
    fn new() -> Self {
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();
        let debug_render = DebugRenderPipeline::default();

        Self {
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            rigid_body_set,
            collider_set,
            debug_render,
            collision_handler: CollisionHandler::default(),
        }
    }

    /// Advance the simulation by one frame.
    #[rune::function]
    fn tick(&mut self, gravity: &Vec2) -> Vec<Collision> {
        {
            let mut list = self.collision_handler.event_list.lock().unwrap();
            list.clear();
        }

        self.physics_pipeline.step(
            &vector![gravity.x, gravity.y],
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &self.collision_handler,
        );

        let list = self.collision_handler.event_list.lock().unwrap();

        list.to_vec()
    }

    /// Draw a debug visualization of the internal state.
    #[rune::function]
    fn draw_debug(&mut self, frame: &mut Frame) {
        self.debug_render.render(
            &mut DebugRender(frame),
            &self.rigid_body_set,
            &self.collider_set,
            &self.impulse_joint_set,
            &self.multibody_joint_set,
            &self.narrow_phase,
        );
    }
}

#[allow(dead_code)]
struct DebugRender<'a>(&'a mut crate::module::video::Frame);

fn hsl_to_rgb(h: f32, s: f32, l: f32, a: f32) -> Color {
    let h = h / 360.0;

    let hue_to_rgb = |p, q, mut t| {
        if t < 0.0 {
            t += 1.0;
        };
        if t > 1.0 {
            t -= 1.0;
        };
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        };
        if t < 1.0 / 2.0 {
            return q;
        };
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        };

        p
    };

    let mut r = l;
    let mut g = l;
    let mut b = l;

    if s != 0.0 {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    Color::rust_new(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    )
}

impl<'a> DebugRenderBackend for DebugRender<'a> {
    fn draw_line(&mut self, _: DebugRenderObject, a: Point<f32>, b: Point<f32>, color: DebugColor) {
        self.0.rust_draw_line(
            &Vec2::rust_new(a.x, a.y),
            &Vec2::rust_new(b.x, b.y),
            1,
            &hsl_to_rgb(color[0], color[1], color[2], color[3]),
        );
    }
}

#[derive(Default)]
struct CollisionHandler {
    event_list: Arc<Mutex<Vec<Collision>>>,
}

impl EventHandler for CollisionHandler {
    fn handle_collision_event(
        &self,
        _: &RigidBodySet,
        _: &ColliderSet,
        event: CollisionEvent,
        _: Option<&ContactPair>,
    ) {
        let mut lock = self.event_list.lock().unwrap();
        match event {
            // TO-DO send event flag as well.
            CollisionEvent::Started(collider_handle_a, collider_handle_b, collision_flag) => {
                lock.push(Collision {
                    handle_a: Solid::rust_new(collider_handle_a),
                    handle_b: Solid::rust_new(collider_handle_b),
                    start: true,
                    sensor: collision_flag.contains(CollisionEventFlags::SENSOR),
                    remove: collision_flag.contains(CollisionEventFlags::REMOVED),
                });
            }
            CollisionEvent::Stopped(collider_handle_a, collider_handle_b, collision_flag) => {
                lock.push(Collision {
                    handle_a: Solid::rust_new(collider_handle_a),
                    handle_b: Solid::rust_new(collider_handle_b),
                    start: false,
                    sensor: collision_flag.contains(CollisionEventFlags::SENSOR),
                    remove: collision_flag.contains(CollisionEventFlags::REMOVED),
                });
            }
        }
    }

    fn handle_contact_force_event(
        &self,
        _: f32,
        _: &RigidBodySet,
        _: &ColliderSet,
        _: &ContactPair,
        _: f32,
    ) {
    }
}

//================================================================

/// A handle to a collision event.
#[derive(Any, Clone, Default)]
#[rune(item = ::physical)]
struct Collision {
    /// Handle to solid body 'A' in the collision event.
    #[rune(get, copy)]
    handle_a: Solid,
    /// Handle to solid body 'B' in the collision event.
    #[rune(get, copy)]
    handle_b: Solid,
    /// Start flag; whether or not the collision event is an "enter" intersection or "exit" intersection.
    #[rune(get)]
    start: bool,
    /// Sensor flag; either solid body 'A' and/or solid body 'B' are a sensor.
    #[rune(get)]
    sensor: bool,
    /// Remove flag; solid body 'A' or solid body 'B' have been removed from the simulation. Trying to use 'A' or 'B' at all might do nothing.
    #[rune(get)]
    remove: bool,
}

impl Collision {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        Ok(())
    }
}

//================================================================

/// Intersection testing.
#[derive(Any)]
#[rune(item = ::physical)]
struct Intersect {}

impl Intersect {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::box_2_box_2)?;
        module.function_meta(Self::box_2_circle)?;
        module.function_meta(Self::circle_circle)?;
        module.function_meta(Self::vec_2_box_2)?;
        module.function_meta(Self::vec_2_circle)?;

        Ok(())
    }

    /// Check if a 2-D box and another 2-D box are intersecting.
    #[rune::function(path = Self::box_2_box_2)]
    fn box_2_box_2(a: &Box2, b: &Box2) -> bool {
        a.point.x < b.scale.x
            && a.scale.x > b.point.x
            && a.point.y > b.scale.y
            && a.scale.y < b.point.y
    }

    /// Check if a 2-D box and a circle are intersecting.
    #[rune::function(path = Self::box_2_circle)]
    fn box_2_circle(a: &Box2, b: &Box2) -> bool {
        todo!()
    }

    /// Check if a circle and another circle are intersecting.
    #[rune::function(path = Self::circle_circle)]
    fn circle_circle(a_point: &Vec2, a_radius: f32, b_point: &Vec2, b_radius: f32) -> bool {
        let distance = (a_point.x - b_point.x).powi(2) + (a_point.y - b_point.y).powi(2);

        distance >= (a_radius - b_radius).powi(2) && distance <= (a_radius + b_radius).powi(2)
    }

    /// Check if a 2-D vector and a 2-D box are intersecting.
    #[rune::function(path = Self::vec_2_box_2)]
    fn vec_2_box_2(a: &Vec2, b: &Box2) -> bool {
        (a.x >= b.point.x && a.x <= b.point.x + b.scale.x)
            && (a.y >= b.point.y && a.y <= b.point.y + b.scale.y)
    }

    /// Check if a 2-D vector and a circle are intersecting.
    #[rune::function(path = Self::vec_2_circle)]
    fn vec_2_circle(a: &Vec2, b_point: &Vec2, b_radius: f32) -> bool {
        (a.x - b_point.x).powi(2) + (a.y - b_point.y).powi(2) < b_radius.powi(2)
    }
}

//================================================================

/// A handle to a solid body.
#[derive(Any, Copy, Clone, Default)]
#[rune(item = ::physical)]
struct Solid {
    #[allow(dead_code)]
    inner: ColliderHandle,
}

impl Solid {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new_cuboid)?;
        module.function_meta(Self::new_ball)?;
        module.function_meta(Self::get_point)?;
        module.function_meta(Self::set_point)?;
        module.function_meta(Self::get_angle)?;
        module.function_meta(Self::set_angle)?;
        module.function_meta(Self::get_parent)?;
        module.function_meta(Self::get_sensor)?;
        module.function_meta(Self::set_sensor)?;
        module.function_meta(Self::get_mass)?;
        module.function_meta(Self::set_mass)?;
        module.function_meta(Self::get_user_data)?;
        module.function_meta(Self::set_user_data)?;
        module.function_meta(Self::remove)?;

        Ok(())
    }

    fn rust_new(inner: ColliderHandle) -> Self {
        Self { inner }
    }

    //================================================================

    /// Create a new solid body instance (cuboid).
    #[rune::function(path = Self::new_cuboid)]
    fn new_cuboid(physical: &mut Physical2D, half: &Vec2, parent: Option<Ref<Rigid>>) -> Self {
        let inner = ColliderBuilder::cuboid(half.x, half.y)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .active_collision_types(ActiveCollisionTypes::all())
            .build();

        let inner = if let Some(parent) = parent {
            physical.collider_set.insert_with_parent(
                inner,
                parent.inner,
                &mut physical.rigid_body_set,
            )
        } else {
            physical.collider_set.insert(inner)
        };

        Self { inner }
    }

    /// Create a new solid body instance (ball).
    #[rune::function(path = Self::new_ball)]
    fn new_ball(physical: &mut Physical2D, scale: f32, parent: Option<Ref<Rigid>>) -> Self {
        let inner = ColliderBuilder::ball(scale).build();

        let inner = if let Some(parent) = parent {
            physical.collider_set.insert_with_parent(
                inner,
                parent.inner,
                &mut physical.rigid_body_set,
            )
        } else {
            physical.collider_set.insert(inner)
        };

        Self { inner }
    }

    //================================================================

    /// Get the point of a solid body.
    #[rune::function]
    fn get_point(&self, physical: &Physical2D) -> Vec2 {
        let inner = physical.collider_set.get(self.inner).unwrap();
        let value = inner.translation();

        Vec2::rust_new(value.x, value.y)
    }

    /// Set the point of a solid body.
    #[rune::function]
    fn set_point(&self, physical: &mut Physical2D, point: &Vec2) {
        let inner = physical.collider_set.get_mut(self.inner).unwrap();

        inner.set_translation(vector![point.x, point.y]);
    }

    /// Get the angle of a solid body.
    #[rune::function]
    fn get_angle(&self, physical: &Physical2D) -> f32 {
        let inner = physical.collider_set.get(self.inner).unwrap();
        let value = inner.rotation();

        value.re
    }

    /// Set the angle of a solid body.
    #[rune::function]
    fn set_angle(&self, physical: &mut Physical2D, angle: f32) {
        let inner = physical.collider_set.get_mut(self.inner).unwrap();

        inner.set_rotation(Rotation::new(angle));
    }

    /// Get the rigid body parent of a solid body.
    #[rune::function]
    fn get_parent(&self, physical: &Physical2D) -> Option<Rigid> {
        if let Some(inner) = physical.collider_set.get(self.inner) {
            if let Some(parent) = inner.parent() {
                return Some(Rigid { inner: parent });
            }
        } else {
            println!("Failed to get solid!");
        }

        None
    }

    /// Check if a solid body is a sensor.
    #[rune::function]
    fn get_sensor(&self, physical: &Physical2D) -> bool {
        let inner = physical.collider_set.get(self.inner).unwrap();

        inner.is_sensor()
    }

    /// Set a solid body as a sensor.
    #[rune::function]
    fn set_sensor(&self, physical: &mut Physical2D, sensor: bool) {
        let inner = physical.collider_set.get_mut(self.inner).unwrap();

        inner.set_sensor(sensor);
    }

    /// Get the mass of a solid body.
    #[rune::function]
    fn get_mass(&self, physical: &Physical2D) -> f32 {
        let inner = physical.collider_set.get(self.inner).unwrap();

        inner.mass()
    }

    /// Set the mass of a solid body.
    #[rune::function]
    fn set_mass(&self, physical: &mut Physical2D, mass: f32) {
        let inner = physical.collider_set.get_mut(self.inner).unwrap();

        inner.set_mass(mass);
    }

    /// Get the user data of a solid body.
    #[rune::function]
    fn get_user_data(&self, physical: &Physical2D) -> u128 {
        let inner = physical.collider_set.get(self.inner).unwrap();

        inner.user_data
    }

    /// Set the user data of a solid body.
    #[rune::function]
    fn set_user_data(&self, physical: &mut Physical2D, user_data: u128) {
        let inner = physical.collider_set.get_mut(self.inner).unwrap();

        inner.user_data = user_data;
    }

    /// Remove a solid body.
    ///
    /// # Warning!
    /// Using this solid body after this call will throw an error.
    #[rune::function]
    fn remove(self, physical: &mut Physical2D, wake_parent: bool) {
        physical.collider_set.remove(
            self.inner,
            &mut physical.island_manager,
            &mut physical.rigid_body_set,
            wake_parent,
        );
    }
}

//================================================================

/// A handle to a rigid body.
#[derive(Any, Copy, Clone)]
#[rune(item = ::physical)]
struct Rigid {
    #[allow(dead_code)]
    inner: RigidBodyHandle,
}

impl Rigid {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new_dynamic)?;
        module.function_meta(Self::new_static)?;
        module.function_meta(Self::new_kinematic)?;
        module.function_meta(Self::get_point)?;
        module.function_meta(Self::set_point)?;
        module.function_meta(Self::get_angle)?;
        module.function_meta(Self::set_angle)?;
        module.function_meta(Self::get_user_data)?;
        module.function_meta(Self::set_user_data)?;
        module.function_meta(Self::remove)?;

        Ok(())
    }

    //================================================================

    /// Create a new rigid body instance (dynamic).
    #[rune::function(path = Self::new_dynamic)]
    fn new_dynamic(physical: &mut Physical2D) -> Self {
        let inner = RigidBodyBuilder::dynamic().build();
        let inner = physical.rigid_body_set.insert(inner);

        Self { inner }
    }

    /// Create a new rigid body instance (static).
    #[rune::function(path = Self::new_static)]
    fn new_static(physical: &mut Physical2D) -> Self {
        let inner = RigidBodyBuilder::fixed().build();
        let inner = physical.rigid_body_set.insert(inner);

        Self { inner }
    }

    /// Create a new rigid body instance (kinematic).
    ///
    /// If `position_kinematic` is set to `true`, the simulation will never automatically modify its position, with the velocity being automatically set
    /// in correspondance with the set position by the user. The opposite is true if the `position_kinematic` is set to `false`.
    /// For more info, click [here.](https://rapier.rs/docs/user_guides/rust/rigid_bodies#rigid-body-type)
    #[rune::function(path = Self::new_kinematic)]
    fn new_kinematic(physical: &mut Physical2D, position_kinematic: bool) -> Self {
        let inner = {
            if position_kinematic {
                RigidBodyBuilder::kinematic_position_based().build()
            } else {
                RigidBodyBuilder::kinematic_velocity_based().build()
            }
        };

        let inner = physical.rigid_body_set.insert(inner);

        Self { inner }
    }

    //================================================================

    /// Get the point of a rigid body.
    #[rune::function]
    fn get_point(&self, physical: &Physical2D) -> Vec2 {
        let inner = physical.rigid_body_set.get(self.inner).unwrap();
        let value = inner.translation();

        Vec2::rust_new(value.x, value.y)
    }

    /// Set the point of a rigid body.
    #[rune::function]
    fn set_point(&self, physical: &mut Physical2D, point: &Vec2, wake: bool) {
        let inner = physical.rigid_body_set.get_mut(self.inner).unwrap();

        inner.set_translation(vector![point.x, point.y], wake);
    }

    /// Get the angle of a rigid body.
    #[rune::function]
    fn get_angle(&self, physical: &Physical2D) -> f32 {
        let inner = physical.rigid_body_set.get(self.inner).unwrap();
        let value = inner.rotation();

        value.re
    }

    /// Set the angle of a rigid body.
    #[rune::function]
    fn set_angle(&self, physical: &mut Physical2D, angle: f32, wake: bool) {
        let inner = physical.rigid_body_set.get_mut(self.inner).unwrap();

        inner.set_rotation(Rotation::new(angle), wake);
    }

    /// Get the user data of a rigid body.
    #[rune::function]
    fn get_user_data(&self, physical: &Physical2D) -> u128 {
        let inner = physical.rigid_body_set.get(self.inner).unwrap();

        inner.user_data
    }

    /// Set the user data of a rigid body.
    #[rune::function]
    fn set_user_data(&self, physical: &mut Physical2D, user_data: u128) {
        let inner = physical.rigid_body_set.get_mut(self.inner).unwrap();

        inner.user_data = user_data;
    }

    /// Remove a rigid body.
    ///
    /// # Warning!
    /// Using this solid body after this call will throw an error.
    #[rune::function]
    fn remove(self, physical: &mut Physical2D, remove_collider: bool) {
        physical.rigid_body_set.remove(
            self.inner,
            &mut physical.island_manager,
            &mut physical.collider_set,
            &mut physical.impulse_joint_set,
            &mut physical.multibody_joint_set,
            remove_collider,
        );
    }
}

//================================================================

/// A handle to a character controller.
#[derive(Any, Copy, Clone)]
#[rune(item = ::physical)]
struct Controller {
    #[allow(dead_code)]
    inner: KinematicCharacterController,
}

impl Controller {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::set_up)?;
        module.function_meta(Self::movement)?;

        Ok(())
    }

    //================================================================

    /// Create a new character controller instance.
    #[rune::function(path = Self::new)]
    fn new() -> Self {
        Self {
            inner: KinematicCharacterController::default(),
        }
    }

    /// Set the "up" vector.
    #[rune::function]
    fn set_up(&mut self, direction: &Vec2) {
        self.inner.up = UnitVector::new_normalize(vector![direction.x, direction.y]);
    }

    /// Move the character controller.
    #[rune::function]
    fn movement(
        &self,
        physical: &mut Physical2D,
        point: &Vec2,
        solid: &Solid,
        rigid: &Rigid,
        // TO-DO formalize this as a structure
    ) -> (bool, bool, Vec2, Vec<Solid>) {
        let mut collision_list_rune = Vec::new();
        let mut collision_list_rust = Vec::new();

        let solid = physical.collider_set.get(solid.inner).unwrap();

        let corrected_movement = self.inner.move_shape(
            // TO-DO pass time-step as parameter.
            60.0,
            &physical.rigid_body_set,
            &physical.collider_set,
            &physical.query_pipeline,
            solid.shape(),
            solid.position(),
            vector![point.x, point.y],
            // TO-DO allow not passing the rigid body.
            QueryFilter::default()
                .exclude_rigid_body(rigid.inner)
                .exclude_sensors(),
            |collision| {
                // TO-DO more information should be sent to Rune about a character collision.
                collision_list_rune.push(Solid::rust_new(collision.handle));
                collision_list_rust.push(collision);
            },
        );

        self.inner.solve_character_collision_impulses(
            60.0,
            &mut physical.rigid_body_set,
            &physical.collider_set,
            &physical.query_pipeline,
            solid.shape(),
            solid.mass(),
            &collision_list_rust,
            QueryFilter::default(),
        );

        (
            corrected_movement.grounded,
            corrected_movement.is_sliding_down_slope,
            Vec2::rust_new(
                corrected_movement.translation.x,
                corrected_movement.translation.y,
            ),
            collision_list_rune,
        )
    }
}

//================================================================

#[rune::module(::physical)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    Physical2D::module(&mut module)?;
    Collision::module(&mut module)?;
    Intersect::module(&mut module)?;
    Solid::module(&mut module)?;
    Rigid::module(&mut module)?;
    Controller::module(&mut module)?;

    Ok(module)
}
