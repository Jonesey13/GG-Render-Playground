use gg::rendering::{Renderable, Polygon};
use ::rendering::{GamePrimitive, Line};
use na;
use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use gg::rendering::render_by_shaders::GliumStandardPrimitive;
use gg::rendering::shaders::Shaders;
use gg::geometry::Polynomial2d;
use glium;
use num::Zero;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct SpikyCircle {
    pub pos: Vector3<f64>,
    pub rad: f64,
    pub spike_height: f64,
    pub num_spikes: usize,
    pub rot: Rotation2<f64>,
    pub color: Vector4<f64>,
    pub has_boundary: bool,
    pub boundary_thickness: f64,
    pub boundary_color: Vector4<f64>,
    pub fixed: bool
}

impl SpikyCircle {
    /// Intended for Standalone use
    pub fn new (
        pos: Vector3<f64>,
        rad: f64,
        spike_height: f64,
        num_spikes: usize,
        angle: f64,
        color: Vector4<f64>,
        fixed: bool
    ) -> SpikyCircle {
        SpikyCircle { 
            pos,
            rad,
            spike_height,
            num_spikes,
            rot: Rotation2::new(angle),
            color,
            has_boundary: false,
            boundary_thickness: 0.0,
            boundary_color: Vector4::zero(),
            fixed
        }
    }

    pub fn new_with_boundary (
        pos: Vector3<f64>,
        rad: f64,
        spike_height: f64,
        num_spikes: usize,
        angle: f64,
        color: Vector4<f64>,
        boundary_thickness: f64,
        boundary_color: Vector4<f64>,
        fixed: bool
    ) -> SpikyCircle {
        SpikyCircle { 
            pos,
            rad,
            spike_height,
            num_spikes,
            rot: Rotation2::new(angle),
            color,
            has_boundary: true,
            boundary_thickness,
            boundary_color,
            fixed
        }
    }

    pub fn get_central_polygon(&self) -> Polygon {
        let mut corners = vec![];

        for i in 0..self.num_spikes {
            let current_spike_root_angle = 1.0 / (self.num_spikes as f64) * (i as f64);
            let corner = self.rad * (Rotation2::new(2.0 * PI * current_spike_root_angle) * Vector2::x());
            corners.push(corner);
        }

        Polygon::new_regular(corners, Vector2::zero(), self.pos, self.color, self.fixed)
    }

    pub fn get_spikes(&self) -> Vec<Polygon> {
        let mut spikes = vec![];

        for i in 0..self.num_spikes {
            let current_spike_root_angle = 1.0 / (self.num_spikes as f64) * (i as f64);
            let next_spike_root_angle = 1.0 / ((self.num_spikes) as f64) * ((i + 1) as f64);
            let current_corner = self.rad * (Rotation2::new(2.0 * PI * current_spike_root_angle) * Vector2::x());
            let next_corner = self.rad * (Rotation2::new(2.0 * PI * next_spike_root_angle) * Vector2::x());

            let mut mid_point = (current_corner + next_corner) / 2.0;
            let mid_point_length = mid_point.norm();
            let spike_point = ((mid_point_length + self.spike_height) / mid_point_length) * mid_point;

            let corners = vec![current_corner, spike_point, next_corner];
            
            spikes.push(
                Polygon::new_regular(corners, mid_point, self.pos, self.color, self.fixed)
            );
        }

        spikes
    }

    pub fn get_lines(&self) -> Vec<Line> {
        let mut lines = vec![];

        for i in 0..self.num_spikes {
            let current_spike_root_angle = 1.0 / (self.num_spikes as f64) * (i as f64);
            let next_spike_root_angle = 1.0 / ((self.num_spikes) as f64) * ((i + 1) as f64);
            let local_pos_2d = Vector2::new(self.pos.x, self.pos.y);
            let current_corner = self.rad * (Rotation2::new(2.0 * PI * current_spike_root_angle) * Vector2::x());
            let next_corner = self.rad * (Rotation2::new(2.0 * PI * next_spike_root_angle) * Vector2::x());

            let mut mid_point = (current_corner + next_corner) / 2.0;
            let mid_point_length = mid_point.norm();
            let spike_point = ((mid_point_length + self.spike_height) / mid_point_length) * mid_point;
            
            lines.push(
                Line::new_rounded(
                    current_corner + local_pos_2d, 
                    spike_point + local_pos_2d, 
                    self.boundary_thickness, 
                    self.boundary_color, 
                    self.pos.z - 0.001, 
                    self.fixed),
            );
            lines.push(
                Line::new_rounded(
                    spike_point + local_pos_2d, 
                    next_corner + local_pos_2d, 
                    self.boundary_thickness, 
                    self.boundary_color, 
                    self.pos.z - 0.001, 
                    self.fixed),
            );
        }

        lines
    }
}

impl Renderable<GamePrimitive> for SpikyCircle {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { 
        let mut output: Vec<GamePrimitive> = self.get_spikes()
        .into_iter()
        .flat_map(|mut spike| {spike.get_primitives()})
        .collect();

        if self.has_boundary {
            let mut lines: Vec<GamePrimitive> = self.get_lines()
                .into_iter()
                .flat_map(|mut line| {line.get_primitives()})
                .collect();
            output.append(&mut lines);
        }

        output.append(&mut self.get_central_polygon().get_primitives());
        output
    }
}