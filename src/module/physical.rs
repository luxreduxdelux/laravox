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

use crate::module::{
    general::{Color, Vec2},
    video::Frame,
};

//================================================================

use rapier2d::{control::KinematicCharacterController, prelude::*};
use rune::{Any, Module, Ref};

//================================================================

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
}

impl Physical2D {
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
        }
    }

    #[rune::function]
    fn tick(&mut self, gravity: &Vec2) {
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
            &(),
        );
    }

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

impl<'a> DebugRenderBackend for DebugRender<'a> {
    fn draw_line(
        &mut self,
        _object: DebugRenderObject,
        a: Point<f32>,
        b: Point<f32>,
        _color: DebugColor,
    ) {
        self.0.rust_draw_line(
            &Vec2::rust_new(a.x, a.y),
            &Vec2::rust_new(b.x, b.y),
            1,
            &Color::rust_new(0, 255, 0, 255),
        );
    }
}

//================================================================

#[derive(Any, Copy, Clone)]
#[rune(item = ::physical)]
struct Collider {
    #[allow(dead_code)]
    inner: ColliderHandle,
}

impl Collider {
    #[rune::function(path = Self::new_cuboid)]
    fn new_cuboid(physical: &mut Physical2D, half: &Vec2, parent: Option<Ref<Rigid>>) -> Self {
        let inner = ColliderBuilder::cuboid(half.x, half.y).build();

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

    #[rune::function]
    fn get_point(&self, physical: &Physical2D) -> Vec2 {
        let inner = physical.collider_set.get(self.inner).unwrap();
        let value = inner.translation();

        Vec2::rust_new(value.x, value.y)
    }

    #[rune::function]
    fn set_point(&self, physical: &mut Physical2D, point: &Vec2) {
        let inner = physical.collider_set.get_mut(self.inner).unwrap();

        inner.set_translation(vector![point.x, point.y]);
    }

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

#[derive(Any, Copy, Clone)]
#[rune(item = ::physical)]
struct Rigid {
    #[allow(dead_code)]
    inner: RigidBodyHandle,
}

impl Rigid {
    #[rune::function(path = Self::new_dynamic)]
    fn new_dynamic(physical: &mut Physical2D) -> Self {
        let inner = RigidBodyBuilder::dynamic().build();
        let inner = physical.rigid_body_set.insert(inner);

        Self { inner }
    }

    #[rune::function(path = Self::new_static)]
    fn new_static(physical: &mut Physical2D) -> Self {
        let inner = RigidBodyBuilder::fixed().build();
        let inner = physical.rigid_body_set.insert(inner);

        Self { inner }
    }

    #[rune::function]
    fn get_point(&self, physical: &Physical2D) -> Vec2 {
        let inner = physical.rigid_body_set.get(self.inner).unwrap();
        let value = inner.translation();

        Vec2::rust_new(value.x, value.y)
    }

    #[rune::function]
    fn get_angle(&self, physical: &Physical2D) -> f32 {
        let inner = physical.rigid_body_set.get(self.inner).unwrap();
        let value = inner.rotation();

        value.re
    }

    #[rune::function]
    fn set_point(&self, physical: &mut Physical2D, point: &Vec2, wake: bool) {
        let inner = physical.rigid_body_set.get_mut(self.inner).unwrap();

        inner.set_translation(vector![point.x, point.y], wake);
    }

    #[rune::function]
    fn set_angle(&self, physical: &mut Physical2D, angle: f32, wake: bool) {
        let inner = physical.rigid_body_set.get_mut(self.inner).unwrap();

        inner.set_rotation(Rotation::new(angle), wake);
    }

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

#[derive(Any, Copy, Clone)]
#[rune(item = ::physical)]
struct Controller {
    #[allow(dead_code)]
    inner: KinematicCharacterController,
}

impl Controller {
    #[rune::function(path = Self::new)]
    fn new() -> Self {
        Self {
            inner: KinematicCharacterController::default(),
        }
    }

    #[rune::function]
    fn set_up(&mut self, direction: &Vec2) {
        self.inner.up = UnitVector::new_normalize(vector![direction.x, direction.y]);
    }

    #[rune::function]
    fn movement(
        &self,
        physical: &Physical2D,
        point: &Vec2,
        collider: &Collider,
        rigid: &Rigid,
        // TO-DO formalize this as a structure
    ) -> (bool, bool, Vec2) {
        let collider = physical.collider_set.get(collider.inner).unwrap();

        let corrected_movement = self.inner.move_shape(
            60.0,                     // The timestep length (can be set to SimulationSettings::dt).
            &physical.rigid_body_set, // The RigidBodySet.
            &physical.collider_set,   // The ColliderSet.
            &physical.query_pipeline, // The QueryPipeline.
            collider.shape(),         // The character’s shape.
            collider.position(),      // The character’s initial position.
            vector![point.x, point.y],
            QueryFilter::default()
                // Make sure the character we are trying to move isn’t considered an obstacle.
                .exclude_rigid_body(rigid.inner),
            |_| {}, // We don’t care about events in this example.
        );

        (
            corrected_movement.grounded,
            corrected_movement.is_sliding_down_slope,
            Vec2::rust_new(
                corrected_movement.translation.x,
                corrected_movement.translation.y,
            ),
        )
    }
}

//================================================================

#[rune::module(::physical)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    module.ty::<Physical2D>()?;
    module.function_meta(Physical2D::new)?;
    module.function_meta(Physical2D::tick)?;
    module.function_meta(Physical2D::draw_debug)?;

    module.ty::<Collider>()?;
    module.function_meta(Collider::new_cuboid)?;
    module.function_meta(Collider::get_point)?;
    module.function_meta(Collider::set_point)?;
    module.function_meta(Collider::remove)?;

    module.ty::<Rigid>()?;
    module.function_meta(Rigid::new_dynamic)?;
    module.function_meta(Rigid::new_static)?;
    module.function_meta(Rigid::get_point)?;
    module.function_meta(Rigid::get_angle)?;
    module.function_meta(Rigid::set_point)?;
    module.function_meta(Rigid::set_angle)?;
    module.function_meta(Rigid::remove)?;

    module.ty::<Controller>()?;
    module.function_meta(Controller::new)?;
    module.function_meta(Controller::set_up)?;
    module.function_meta(Controller::movement)?;

    Ok(module)
}
