pub mod child;
pub mod lmao;

use crate::render::color::RgbColor;
use crate::render::draw2d::DrawContext2D;
use crate::resolve;
use crate::ui::attributes::Attributes;
use crate::ui::elements::child::Child;
use crate::ui::styles::{
    ChildAlign, Dimension, Direction, Origin, Point, Position, ResCon, TextFit, UiStyle, UiValue,
};
use crate::ui::styles::Resolve;
use mvutils::unsafe_utils::Unsafe;
use mvutils::utils::{Recover, RwArc, TetrahedronOp};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc};
use parking_lot::RwLock;

pub trait UiElementCallbacks {
    fn init(&mut self);

    fn draw(&mut self, ctx: &mut DrawContext2D);
}

pub trait UiElement: UiElementCallbacks {
    fn new(attributes: Attributes, style: UiStyle) -> Self
    where
        Self: Sized;

    fn attributes(&self) -> &Attributes;

    fn attributes_mut(&mut self) -> &Attributes;

    fn state(&self) -> &UiElementState;

    fn state_mut(&mut self) -> &mut UiElementState;

    fn style(&self) -> &UiStyle;

    fn style_mut(&mut self) -> &mut UiStyle;

    fn components(&self) -> (&Attributes, &UiStyle, &UiElementState);

    fn components_mut(&mut self) -> (&mut Attributes, &mut UiStyle, &mut UiElementState);

    fn add_child(&mut self, child: Child) {
        self.state_mut().children.push(child);
    }

    fn children(&self) -> &[Child] {
        &self.state().children
    }

    fn children_mut(&mut self) -> &mut [Child] {
        &mut self.state_mut().children
    }

    fn get_size(&self, s: &str) -> Dimension<i32>;
}

pub(crate) struct UiElementState {
    pub ctx: ResCon,
    pub parent: Option<Box<dyn UiElement>>,

    pub(crate) children: Vec<Child>,

    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) content_x: i32,
    pub(crate) content_y: i32,
    pub(crate) bounding_x: i32,
    pub(crate) bounding_y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) content_width: i32,
    pub(crate) content_height: i32,
    pub(crate) bounding_width: i32,
    pub(crate) bounding_height: i32,

    pub(crate) transforms: UiTransformations,
}

#[derive(Clone)]
pub(crate) struct UiTransformations {
    translation: Dimension<i32>,
    rotation: f32,
    scale: Dimension<f32>,
    origin: Origin,
}

impl UiElementState {
    pub(crate) fn new() -> Self {
        Self {
            ctx: ResCon { dpi: 0.0 },
            parent: None,
            children: vec![],
            x: 0,
            y: 0,
            content_x: 0,
            content_y: 0,
            bounding_x: 0,
            bounding_y: 0,
            width: 0,
            height: 0,
            content_width: 0,
            content_height: 0,
            bounding_width: 0,
            bounding_height: 0,
            transforms: UiTransformations {
                translation: Dimension::new(0, 0),
                rotation: 0.0,
                scale: Dimension::new(1.0, 1.0),
                origin: Origin::Center,
            },
        }
    }

    pub fn comp(elem: Arc<RwLock<dyn UiElement>>, ctx: &mut DrawContext2D) {
        let mut guard = elem.write();
        guard.state_mut().ctx.dpi = ctx.dpi();
        let (_, style, state) = guard.components_mut();

        let s = "Hello";

        let size = guard.get_size(s);

        let text_fit = if style.text.fit.is_set() {
            resolve!(guard, text.fit)
        } else {
            if matches!(style.text.fit, UiValue::None) {
                TextFit::ExpandParent
            } else {
                TextFit::CropText
            }
        };
    }

    pub fn compute(elem: Arc<RwLock<dyn UiElement>>, ctx: &mut DrawContext2D) {
        let mut guard = elem.write();
        guard.state_mut().ctx.dpi = ctx.dpi();
        let (_, style, state) = guard.components_mut();

        let direction = if style.direction.is_set() {
            resolve!(guard, direction)
        } else {
            Direction::default()
        };

        let origin = if style.origin.is_set() {
            resolve!(guard, origin)
        } else {
            Origin::BottomLeft
        };

        let child_align = if style.child_align.is_set() {
            resolve!(guard, child_align)
        } else {
            ChildAlign::Start
        };

        let width_auto = style.width.is_auto();
        let height_auto = style.height.is_auto();

        let mut width = if style.width.is_set() {
            resolve!(guard, width)
        } else {
            0
        };
        let mut height = if style.height.is_set() {
            resolve!(guard, height)
        } else {
            0
        };

        if width_auto || height_auto {
            let mut occupied_width = 0;
            let mut occupied_height = 0;

            for child in &mut state.children {
                if let Child::Element(ref mut e) = child {
                    let stat = e.state_mut();

                    if matches!(direction, Direction::Horizontal) {
                        stat.x = occupied_width;
                    } else {
                        stat.y = occupied_height;
                    }

                    UiElementState::compute(elem.clone(), ctx);

                    if matches!(direction, Direction::Horizontal) {
                        occupied_width += e.state().bounding_width;
                        occupied_height = occupied_height.max(e.state().bounding_height);
                    } else {
                        occupied_width = occupied_width.max(e.state().bounding_width);
                        occupied_height += e.state().bounding_height;
                    }
                } else {
                    let s = child.as_string();

                    let text_fit = if style.text.fit.is_set() {
                        resolve!(guard, text.fit)
                    } else {
                        if matches!(style.text.fit, UiValue::None) {
                            TextFit::ExpandParent
                        } else {
                            TextFit::CropText
                        }
                    };

                    if matches!(text_fit, TextFit::ExpandParent) {

                        //-----------------------------
                        //HERE IS THE CALL TO get_size()
                        //-----------------------------

                        let size = guard.get_size(&s);

                        if matches!(direction, Direction::Horizontal) {
                            occupied_width += size.width;
                            occupied_height = occupied_height.max(size.height);
                        } else {
                            occupied_width = occupied_width.max(size.width);
                            occupied_height += size.height;
                        }
                    }
                }
            }

            if width_auto && occupied_width > width {
                width = occupied_width;
            }

            if height_auto && occupied_height > height {
                height = occupied_height;
            }
        }

        let padding = style.padding.get(guard.deref()); //t,b,l,r
        let margin = style.margin.get(guard.deref());

        state.content_width = width;
        state.content_height = height;
        state.width = width + padding[2] + padding[3];
        state.height = height + padding[0] + padding[1];
        state.bounding_width = state.width + margin[2] + margin[3];
        state.bounding_height = state.height + margin[0] + margin[1];

        let position = if style.position.is_set() {
            resolve!(guard, position)
        } else {
            Position::Relative
        };
        if matches!(position, Position::Absolute) {
            if !style.x.is_auto() {
                let x = if style.x.is_set() {
                    resolve!(guard, x)
                } else {
                    0
                };
                state.content_x = origin.get_actual_x(x, state.content_width);
            }
            if !style.y.is_auto() {
                let y = if style.y.is_set() {
                    resolve!(guard, y)
                } else {
                    0
                };
                state.content_y = origin.get_actual_y(y, state.content_height);
            }
        }

        state.x = state.content_x - padding[2];
        state.y = state.content_y - padding[1];
        state.bounding_x = state.x - margin[2];
        state.bounding_y = state.y - margin[1];

        for e in state
            .children
            .deref_mut()
            .iter_mut()
            .filter(|c| c.is_element())
            .map(|c| match c {
                Child::Element(ref mut e) => e,
                _ => {
                    unreachable!()
                }
            })
        {
            //set xy of child to rel coords if pos=rel

            let child_position = if e.style().position.is_set() {
                resolve!(e, position)
            } else {
                Position::Relative
            };
            if matches!(child_position, Position::Relative) {
                let stat = e.state_mut();
                let x_off = stat.x;
                let y_off = stat.y;

                let child_origin = if e.style().origin.is_set() {
                    resolve!(e, origin)
                } else {
                    Origin::BottomLeft
                };

                match direction {
                    Direction::Vertical => {
                        stat.content_y = child_origin.get_actual_y(y_off, stat.content_height);
                        stat.content_x = child_origin.get_actual_x(
                            match child_align {
                                ChildAlign::Start => state.content_x,
                                ChildAlign::End => {
                                    state.content_x + state.content_width - stat.content_width
                                }
                                ChildAlign::Middle => {
                                    state.content_x + state.content_width / 2 - stat.content_width / 2
                                }
                                ChildAlign::OffsetStart(o) => state.content_x + o,
                                ChildAlign::OffsetEnd(o) => {
                                    state.content_x + state.content_width - stat.content_width - o
                                }
                                ChildAlign::OffsetMiddle(o) => {
                                    state.content_x + state.content_width / 2 - stat.content_width / 2
                                        + o
                                }
                            },
                            stat.content_width,
                        );
                    }
                    Direction::Horizontal => {
                        stat.content_x = child_origin.get_actual_x(x_off, stat.content_width);
                        stat.content_y = child_origin.get_actual_y(
                            match child_align {
                                ChildAlign::Start => state.content_y,
                                ChildAlign::End => {
                                    state.content_y + state.content_height - stat.content_height
                                }
                                ChildAlign::Middle => {
                                    state.content_y + state.content_height / 2
                                        - stat.content_height / 2
                                }
                                ChildAlign::OffsetStart(o) => state.content_y + o,
                                ChildAlign::OffsetEnd(o) => {
                                    state.content_y + state.content_height - stat.content_height - o
                                }
                                ChildAlign::OffsetMiddle(o) => {
                                    state.content_y + state.content_height / 2
                                        - stat.content_height / 2
                                        + o
                                }
                            },
                            stat.content_height,
                        );
                    }
                }

                let padding = e.style().padding.get(guard.deref()); //t,b,l,r
                let margin = e.style().margin.get(guard.deref());

                stat.x = stat.content_x - padding[2];
                stat.y = stat.content_y - padding[1];
                stat.bounding_x = stat.x - margin[2];
                stat.bounding_y = stat.y - margin[1];
            }
        }
    }
}
