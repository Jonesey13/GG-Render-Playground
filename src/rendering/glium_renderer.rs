use gg::rendering::Renderer;
use gg::rendering::primitives::rectangle::{Rectangle};
use gg::rendering::primitives::circle_part::{CirclePart};
use gg::rendering::primitives::polygon::{Polygon};
use gg::rendering::primitives::text::{TextBuffer, PlainText};
use gg::rendering::primitives::{TextureRect}; 
use ::rendering::primitives::*;
use ::rendering::*;
use gg::rendering::DisplaySettings;
use gg::rendering::glium_buffer::{GliumBuffer, BasicBuffer};
use gg::rendering::WindowSpec;
use glium;
use glium::{Display, Surface, DrawParameters, Depth, DepthTest, Blend};
use glium::texture;
use glium::glutin::EventsLoop;
use na;
use na::Matrix4;
use num::One;
use games::view_details;
use gg::utils::transforms_2d;
use debug::*;
use image;

pub struct GliumRenderer<'a> {
    display: Box<Display>,
    events_loop: Box<EventsLoop>,
    draw_params: DrawParameters<'a>,
    rect_buffer: BasicBuffer<Rectangle>,
    rect_with_gradient_buffer: BasicBuffer<RectWithGradient>,
    texture_rect_buffer: BasicBuffer<TextureRect>,
    circ_buffer: BasicBuffer<CirclePart>,
    polygon_buffer: BasicBuffer<Polygon>,
    bezier_rect_buffer: BasicBuffer<BezierRect>,
    cubic_rect_buffer: BasicBuffer<PaddleRect>,    
    bezier_branch_rect_buffer: BasicBuffer<BezierBranchRect>,
    bezier_branch_circ_buffer: BasicBuffer<BezierBranchCirc>,
    text_processor: TextBuffer<'a, PlainText>,
    view_details: view_details::ViewDetails,
    display_settings: DisplaySettings,
    texture_array: texture::srgb_texture2d_array::SrgbTexture2dArray
}

impl<'a> GliumRenderer<'a> {
    pub fn new(settings: DisplaySettings) -> GliumRenderer<'a> {
        let (display, events_loop) = Self::build_display_and_events_loop(settings);

        let draw_params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLessOrEqual,
                write: true,..Default::default()
            },
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        GliumRenderer {
            display: Box::new(display.clone()),
            events_loop: Box::new(events_loop),
            draw_params: draw_params,
            rect_buffer: BasicBuffer::<Rectangle>::new(&display),
            rect_with_gradient_buffer: BasicBuffer::<RectWithGradient>::new(&display),
            texture_rect_buffer: BasicBuffer::<TextureRect>::new(&display),
            circ_buffer: BasicBuffer::<CirclePart>::new(&display),
            polygon_buffer: BasicBuffer::<Polygon>::new(&display),
            bezier_rect_buffer: BasicBuffer::<BezierRect>::new(&display),
            cubic_rect_buffer: BasicBuffer::<PaddleRect>::new(&display),
            bezier_branch_rect_buffer: BasicBuffer::<BezierBranchRect>::new(&display),
            bezier_branch_circ_buffer: BasicBuffer::<BezierBranchCirc>::new(&display),
            text_processor: TextBuffer::new(&display, settings),
            view_details: view_details::ViewDetails::TwoDim(view_details::ViewDetails2D::default()),
            display_settings: settings,
            texture_array: texture::srgb_texture2d_array::SrgbTexture2dArray::empty(&display, 1024, 1024, 1).unwrap()
        }
    }

    pub fn reset(&mut self, settings: DisplaySettings) {
        let window = Self::build_window(settings, &self.events_loop);
        
        let context = Self::build_context(settings);

        self.display.rebuild(window, context, &self.events_loop).unwrap();
    }

    fn build_display_and_events_loop(settings: DisplaySettings) -> (Display, EventsLoop) {
        let events_loop = glium::glutin::EventsLoop::new();
        
        let window = Self::build_window(settings, &events_loop);
        
        let context = Self::build_context(settings);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        (display, events_loop)
    }

    fn build_window(settings: DisplaySettings, events_loop: &glium::glutin::EventsLoop) -> glium::glutin::WindowBuilder {
        let mut window = glium::glutin::WindowBuilder::new()
            .with_dimensions(settings.res.0, settings.res.1);

        if settings.fullscreen { 
            window = window.with_fullscreen(Some(events_loop.get_primary_monitor())); 
        }

        window
    }

    fn build_context(settings: DisplaySettings) -> glium::glutin::ContextBuilder<'a> {
        glium::glutin::ContextBuilder::new().with_multisampling(settings.multisample_level)
    }

    fn flush_buffers(&mut self) {
        self.rect_buffer.flush_buffer();
        self.rect_with_gradient_buffer.flush_buffer();
        self.texture_rect_buffer.flush_buffer();
        self.circ_buffer.flush_buffer();
        self.polygon_buffer.flush_buffer();
        self.bezier_rect_buffer.flush_buffer();
        self.cubic_rect_buffer.flush_buffer();        
        self.bezier_branch_rect_buffer.flush_buffer();
        self.bezier_branch_circ_buffer.flush_buffer();              
        self.text_processor.flush_buffer();
    }
    
    pub fn create_worldview_mat(view_details: view_details::ViewDetails, aspect_ratio: f64) ->  [[f32;4]; 4] {
        let view_mat = match
            view_details {
                view_details::ViewDetails::TwoDim(ref view) =>
                transforms_2d::build_worldview_mat(
                    view.camera_pos,
                    view.viewport_height,
                    view.viewport_length,
                    aspect_ratio,
                    view.up_vector,
                    view.use_aspect_ratio),
                view_details::ViewDetails::ThreeDim(_) => panic!("3D mode not supported!"),
                _ => Matrix4::one()
            };
        let single_mat: Matrix4<f32> = na::convert(view_mat);
        *single_mat.as_ref()
    }

    pub fn new_with_textures(settings: DisplaySettings, mut image_array: Vec<image::DynamicImage>) -> Self {
        let texture_array = image_array
            .iter_mut()
            .map(|image| {image.to_rgba()})
            .map(|image| {
                let image_dimensions = image.dimensions();
                glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
            })
            .collect();

        let mut renderer = GliumRenderer::new(settings);

        let texture = glium::texture::srgb_texture2d_array::SrgbTexture2dArray::new(renderer.display.as_ref(), texture_array).unwrap();

        renderer.texture_array = texture;
    
        renderer
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    type Primitive = GamePrimitive;

    fn load_renderables(&mut self, renderables: Vec<Box<RenderableTestRenderable>>) {
        debug_clock_start("Render::glium_load");
        for mut renderable in renderables {
            for primitive in renderable.get_primitives() {
                match primitive {
                        GamePrimitive::Rect(rectangle) => self.rect_buffer.load_renderable(rectangle),
                        GamePrimitive::RectWithGradient(rect) => self.rect_with_gradient_buffer.load_renderable(rect),
                        GamePrimitive::Circ(circle) => self.circ_buffer.load_renderable(circle),
                        GamePrimitive::Text(text) => self.text_processor.load_renderable(text),
                        GamePrimitive::BezierRect(bezier_rect) => self.bezier_rect_buffer.load_renderable(bezier_rect),
                        GamePrimitive::PaddleRect(bezier_rect) => self.cubic_rect_buffer.load_renderable(bezier_rect),                        
                        GamePrimitive::BezierBranchRect(bezier_branch_rect) => self.bezier_branch_rect_buffer.load_renderable(bezier_branch_rect),                        
                        GamePrimitive::BezierBranchCirc(bezier_branch_circ) => self.bezier_branch_circ_buffer.load_renderable(bezier_branch_circ),                          
                        GamePrimitive::Poly(polygon) => self.polygon_buffer.load_renderable(polygon),
                        GamePrimitive::TextureRect(rect) => self.texture_rect_buffer.load_renderable(rect),
                }
            }
        }
        debug_clock_stop("Render::glium_load");
    }

    fn render(&mut self) {
        debug_clock_start("Render::glium_render");
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.clear_depth(1.0);

        let (width, height) = target.get_dimensions();
        let aspect_ratio = width as f64 / height as f64;
        
        {
            let uniforms = uniform! {
                screen_width: width,
                screen_height: height,
                aspect_ratio: aspect_ratio as f32,
                world_view: GliumRenderer::create_worldview_mat(self.view_details, aspect_ratio),
                tex: &self.texture_array
            };
            
            self.rect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            self.rect_with_gradient_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);            
            self.texture_rect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            self.circ_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            self.polygon_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);        
            self.bezier_rect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            self.cubic_rect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);            
            self.bezier_branch_rect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            self.bezier_branch_circ_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms); 
            self.text_processor.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            
            target.finish().unwrap();
        }
        
        self.flush_buffers();
        debug_clock_stop("Render::glium_render");
    }

    fn set_worldview(&mut self, view_details: view_details::ViewDetails) {
        self.view_details = view_details;
    }

    fn get_events_loop(&mut self) -> Option<&mut EventsLoop> {
        Some(&mut self.events_loop)
    }

    fn get_window_spec(&self) -> WindowSpec {
        let (width, height) = self.display.gl_window().get_inner_size().unwrap();

        WindowSpec {
            aspect_ratio: width as f64 / height as f64
        }
    }
}
