use na::{Vector2, Vector3, Vector4, Matrix2, Rotation2, convert};
use ::rendering::{Line, Arrow};
use ::rendering::{GamePrimitive };
use gg::rendering::{Renderable, Rectangle};
use gg::geometry;
use std::f64::consts::PI;
use ::constants;

#[derive(Copy, Clone, Debug)]
pub struct StretchItem {
    pos: Vector3<f64>,
    scale: f64,
    select_color: Vector4<f64>,
    rot_angle: f64
}

impl StretchItem {
    pub fn new(
        pos: Vector3<f64>,
        scale: f64,
        rot_angle: f64
    ) -> Self {
        Self {
            pos,
            scale,
            select_color: constants::stretch_item::FOREGROUND_COLOUR.into(),
            rot_angle
        }
    }

    fn get_plus_sign_primitives(&self) -> Vec<GamePrimitive> {
        let hori_rect = Rectangle::new_with_rotation(
            self.scale, 
            constants::stretch_item::RECT_WIDTH * self.scale, 
            self.pos,
            Rotation2::new(2.0 * PI * self.rot_angle),
            constants::stretch_item::BACKGROUND_COLOUR.into(),
            false
        );

        let vert_rect = Rectangle::new_with_rotation(
            self.scale, 
            constants::stretch_item::RECT_WIDTH * self.scale, 
            self.pos,
            Rotation2::new(2.0 * PI * (self.rot_angle + 0.25)),
            constants::stretch_item::BACKGROUND_COLOUR.into(),
            false
        );

        vec![
            GamePrimitive::Rect(hori_rect),
            GamePrimitive::Rect(vert_rect)
        ]
    }

    fn get_icon_renderables(&self) -> Vec<Arrow> {
        let arrow_scale = constants::stretch_item::ARROW_SCALE;
        let arrow_dim: Vector2<f64> = constants::stretch_item::ARROW_DIM.into();
        let twod_pos: Vector2<f64> = Vector2::new(self.pos.x, self.pos.y);
        let rot = Rotation2::new(2.0 * PI * self.rot_angle);

        let right_arrow = Arrow::new_rounded(
            twod_pos,
            twod_pos + rot * (Vector2::x() * arrow_scale * self.scale / 2.0),
            constants::stretch_item::ARROW_THICKNESS * self.scale * arrow_scale,
            arrow_dim * self.scale * arrow_scale,
            self.select_color,
            self.pos.z - 0.001,
            false
        );

        let left_arrow = Arrow::new_rounded(            
            twod_pos,
            twod_pos - rot * (Vector2::x() * arrow_scale * self.scale / 2.0),
            constants::stretch_item::ARROW_THICKNESS * self.scale * arrow_scale,
            arrow_dim * self.scale * arrow_scale,
            self.select_color,
            self.pos.z - 0.001,
            false
        );

        vec![
            right_arrow,
            left_arrow
        ]
    }

    fn get_boundary_lines(&self) -> Vec<Line> {
        let inner_step = self.scale * constants::stretch_item::RECT_WIDTH / 2.0;
        let outer_step = self.scale / 2.0;
        let twod_pos: Vector2<f64> = Vector2::new(self.pos.x, self.pos.y);
        let rot = Rotation2::new(2.0 * PI * self.rot_angle);

        let right_outer_line = geometry::Line::new(Vector2::new(outer_step, inner_step), Vector2::new(outer_step, -inner_step));
        let right_upper_line = geometry::Line::new(Vector2::new(outer_step, inner_step), Vector2::new(inner_step, inner_step));
        let right_lower_line = geometry::Line::new(Vector2::new(outer_step, -inner_step), Vector2::new(inner_step, -inner_step));
        let upper_outer_line = geometry::Line::new(Vector2::new(inner_step, outer_step), Vector2::new(-inner_step, outer_step));
        let upper_right_line = geometry::Line::new(Vector2::new(inner_step, outer_step), Vector2::new(inner_step, inner_step));
        let upper_left_line = geometry::Line::new(Vector2::new(-inner_step, outer_step), Vector2::new(-inner_step, inner_step));
        let left_outer_line = geometry::Line::new(Vector2::new(-outer_step, inner_step), Vector2::new(-outer_step, -inner_step));
        let left_upper_line = geometry::Line::new(Vector2::new(-outer_step, inner_step), Vector2::new(-inner_step, inner_step));
        let left_lower_line = geometry::Line::new(Vector2::new(-outer_step, -inner_step), Vector2::new(-inner_step, -inner_step));
        let lower_outer_line = geometry::Line::new(Vector2::new(inner_step, -outer_step), Vector2::new(-inner_step, -outer_step));
        let lower_right_line = geometry::Line::new(Vector2::new(inner_step, -outer_step), Vector2::new(inner_step, -inner_step));
        let lower_left_line = geometry::Line::new(Vector2::new(-inner_step, -outer_step), Vector2::new(-inner_step, -inner_step));

        let lines = vec![
            right_outer_line,
            right_upper_line,
            right_lower_line,
            upper_outer_line,
            upper_right_line,
            upper_left_line,
            left_outer_line,
            left_upper_line,
            left_lower_line,
            lower_outer_line,
            lower_right_line,
            lower_left_line
        ];

        lines.into_iter().map(|line| {
            Line::new_rounded(
                twod_pos + rot * line.beg, 
                twod_pos + rot * line.end, 
                self.scale * constants::stretch_item::THICKNESS, 
                self.select_color, 
                self.pos.z - 0.001, 
                false)
        }).collect()
    }
}

impl Renderable<GamePrimitive> for StretchItem {
    fn get_primitives(&mut self) -> Vec<GamePrimitive> { 
        let mut output: Vec<GamePrimitive> = self.get_plus_sign_primitives();

        let mut icon_primitives = self.get_icon_renderables().iter_mut().flat_map(|rend| {rend.get_primitives()}).collect();
        output.append(&mut icon_primitives);

        let mut line_primitives = self.get_boundary_lines().iter_mut().flat_map(|rend| {rend.get_primitives()}).collect();
        output.append(&mut line_primitives);
        output
    }
}