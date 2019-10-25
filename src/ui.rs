use ggez::graphics;
use ggez::input;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::GameState;

#[derive(Clone, Builder)]
pub struct Ui {
    #[builder(default = "[0.0,0.0]")]
    origin: [f32; 2],
    #[builder(default = "[1.0,1.0]")]
    size: [f32; 2],

    #[builder(default = "[0.0,0.0,0.0,0.0]")]
    background: [f32; 4],
    #[builder(default = "self.default_background()")]
    background_hover: [f32; 4],
    #[builder(default = "self.default_background()")]
    background_click: [f32; 4],

    #[builder(default = "\"\".to_string()")]
    text: String,
    #[builder(default = "None")]
    font: Option<graphics::Font>,
    #[builder(default = "12.0")]
    font_size: f32,
    #[builder(default = "graphics::Align::Left")]
    align: graphics::Align,

    #[builder(default = "[0.0,0.0,0.0,1.0]")]
    color: [f32; 4],
    #[builder(default = "self.default_color()")]
    color_hover: [f32; 4],
    #[builder(default = "self.default_color()")]
    color_click: [f32; 4],

    #[builder(default = "None")]
    callback: Option<fn(&mut GameState)>,

    #[builder(default = "vec![]")]
    children: Vec<Ui>,
}

impl Ui {
    pub fn draw(&mut self, ctx: &mut Context, parent: Option<graphics::Rect>) -> GameResult {
        let rect = if parent.is_some() {
            let parent_rect = parent.unwrap();
            graphics::Rect::new(
                self.origin[0] * parent_rect.w + parent_rect.x,
                self.origin[1] * parent_rect.h + parent_rect.y,
                self.size[0] * parent_rect.w,
                self.size[1] * parent_rect.h,
            )
        } else {
            let size = graphics::size(ctx);
            let parent_rect = graphics::Rect::new(0.0, 0.0, size.0, size.1);
            graphics::Rect::new(
                self.origin[0] * parent_rect.w + parent_rect.x,
                self.origin[1] * parent_rect.h + parent_rect.y,
                self.size[0] * parent_rect.w,
                self.size[1] * parent_rect.h,
            )
        };
        let focus = self.has_focus(ctx, parent);
        let click = input::mouse::button_pressed(ctx, input::mouse::MouseButton::Left) && focus;
        let (color, background) = if focus && click {
            (self.color_click, self.background_click)
        } else if focus {
            (self.color_hover, self.background_hover)
        } else {
            (self.color, self.background)
        };
        let graphics_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::from(background),
        )?;
        graphics::draw(ctx, &graphics_rect, graphics::DrawParam::default())?;
        if self.font.is_some() && self.text.len() != 0 {
            let text =
                &mut graphics::Text::new((self.text.as_ref(), self.font.unwrap(), self.font_size));
            text.set_bounds(na::Point2::new(rect.w, rect.h), self.align);
            let height = text.height(ctx) as f32;
            graphics::draw(
                ctx,
                text,
                graphics::DrawParam::new()
                    .dest(na::Point2::new(
                        rect.x,
                        rect.y + ((rect.h - height) as f32 / 2.0),
                    ))
                    .color(graphics::Color::from(color)),
            )?;
        }
        for child in self.children.iter_mut() {
            child.draw(ctx, Some(rect))?;
        }
        Ok(())
    }

    pub fn has_focus(&mut self, ctx: &mut Context, parent: Option<graphics::Rect>) -> bool {
        let rect = if parent.is_some() {
            let parent_rect = parent.unwrap();
            graphics::Rect::new(
                self.origin[0] * parent_rect.w + parent_rect.x,
                self.origin[1] * parent_rect.h + parent_rect.y,
                self.size[0] * parent_rect.w,
                self.size[1] * parent_rect.h,
            )
        } else {
            let size = graphics::size(ctx);
            let parent_rect = graphics::Rect::new(0.0, 0.0, size.0, size.1);
            graphics::Rect::new(
                self.origin[0] * parent_rect.w + parent_rect.x,
                self.origin[1] * parent_rect.h + parent_rect.y,
                self.size[0] * parent_rect.w,
                self.size[1] * parent_rect.h,
            )
        };
        let mouse_pos = input::mouse::position(ctx);
        rect.contains(mouse_pos)
    }
}

impl UiBuilder {
    pub fn push(&mut self, child: Ui) -> &mut Self {
        let new = self;
        if new.children.is_some() {
            new.children.as_mut().unwrap().push(child);
        } else {
            new.children = Some(vec![child]);
        }
        new
    }

    pub fn default_background(&self) -> [f32; 4] {
        self.background.clone().unwrap_or([0.0, 0.0, 0.0, 0.0])
    }
    pub fn default_color(&self) -> [f32; 4] {
        self.color.clone().unwrap_or([0.0, 0.0, 0.0, 1.0])
    }
}
