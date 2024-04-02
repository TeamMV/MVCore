use crate::render::color::RgbColor;
use crate::render::text::Font;
use crate::resources::resources::R;
use crate::ui::background::{Background, BackgroundInfo, RectangleBackground};
use crate::ui::elements::UiElement;
use mvutils::unsafe_utils::Unsafe;
use mvutils::utils::{PClamp, TetrahedronOp};
use num_traits::Num;
use std::convert::Infallible;
use std::fmt::Debug;
use std::sync::Arc;
use parking_lot::RwLock;

#[macro_export]
macro_rules! resolve {
    ($elem:ident, $($style:ident).*) => {
        {
            let s = &$elem.style().$($style).*;
            let v: Option<_> = s.resolve($elem.state().ctx.dpi, $elem.state().parent.clone(), |s| {&s.$($style).*});
            if let Some(v) = v {
                v
            }
            else {
                log::error!("UiValue {:?} failed to resolve on element {:?}", stringify!($($style).*), $elem.attributes().id);
                $crate::ui::styles::UiStyle::default().$($style).*
                .resolve($elem.state().ctx.dpi, None, |s| {&s.$($style).*})
                .expect("Default style could not be resolved")
            }
        }
    };
}

#[derive(Clone)]
pub struct UiStyle {
    pub x: LayoutField<i32>,
    pub y: LayoutField<i32>,
    pub width: LayoutField<i32>,
    pub height: LayoutField<i32>,
    pub padding: SideStyle,
    pub margin: SideStyle,
    pub origin: UiValue<Origin>,
    pub position: UiValue<Position>,
    pub rotation_origin: UiValue<Origin>,
    pub rotation: UiValue<f32>,
    pub direction: UiValue<Direction>,
    pub child_align: UiValue<ChildAlign>,

    pub text: TextStyle,

    pub background: BackgroundInfo,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Origin {
    TopLeft,
    #[default]
    BottomLeft,
    TopRight,
    BottomRight,
    Center,
    Custom(i32, i32),
}

impl Origin {
    pub fn is_right(&self) -> bool {
        matches!(self, Origin::BottomRight | Origin::TopRight)
    }

    pub fn is_left(&self) -> bool {
        *self == Origin::BottomLeft || *self == Origin::TopLeft
    }

    pub fn get_custom(&self) -> Option<(i32, i32)> {
        if let Origin::Custom(x, y) = self {
            Some((*x, *y))
        } else {
            None
        }
    }

    pub fn get_actual_x(&self, x: i32, width: i32) -> i32 {
        match self {
            Origin::TopLeft => x,
            Origin::BottomLeft => x,
            Origin::TopRight => x - width,
            Origin::BottomRight => x - width,
            Origin::Center => x - width / 2,
            Origin::Custom(cx, _) => x - cx,
        }
    }

    pub fn get_actual_y(&self, y: i32, height: i32) -> i32 {
        match self {
            Origin::TopLeft => y - height,
            Origin::BottomLeft => y,
            Origin::TopRight => y - height,
            Origin::BottomRight => y,
            Origin::Center => y - height / 2,
            Origin::Custom(_, cy) => y - cy,
        }
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Position {
    Absolute,
    #[default]
    Relative,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    Vertical,
    #[default]
    Horizontal,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum TextFit {
    ExpandParent,
    #[default]
    CropText,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChildAlign {
    #[default]
    Start,
    End,
    Middle,
    OffsetStart(i32),
    OffsetEnd(i32),
    OffsetMiddle(i32),
}

#[derive(Clone)]
pub struct TextStyle {
    pub size: LayoutField<f32>,
    pub kerning: LayoutField<f32>,
    pub skew: LayoutField<f32>,
    pub stretch: UiValue<Dimension<f32>>,
    pub font: UiValue<Arc<Font>>,
    pub fit: UiValue<TextFit>,
}

impl TextStyle {
    pub fn initial() -> Self {
        Self {
            size: UiValue::Measurement(Unit::BarleyCorn(1.0)).into(),
            kerning: UiValue::None.into(),
            skew: UiValue::None.into(),
            stretch: UiValue::None,
            font: UiValue::Auto,
            fit: UiValue::Auto,
        }
    }
}

#[derive(Clone)]
pub struct LayoutField<T: PartialOrd + Clone + 'static> {
    pub value: UiValue<T>,
    pub min: UiValue<T>,
    pub max: UiValue<T>
}

impl<T: PartialOrd + Clone + 'static> Resolve<T> for LayoutField<T> {
    fn resolve<F>(&self, dpi: f32, parent: Option<Arc<RwLock<dyn UiElement>>>, map: F) -> Option<T> where F: Fn(&UiStyle) -> &Self {
        let value = self.value.resolve(dpi, parent.clone(), |s| &map(s).value);
        let min = self.min.resolve(dpi, parent.clone(), |s| &map(s).min);
        let max = self.max.resolve(dpi, parent, |s| &map(s).max);

        if value.is_none() { return None; }

        let mut emin = None;
        let mut emax = None;

        if min.is_none() {
            emin = Some(value.clone().unwrap());
        } else {
            emin = min;
        }

        if max.is_none() {
            emax = Some(value.clone().unwrap());
        } else {
            emax = max;
        }

        Some(value.unwrap().p_clamp(emin.unwrap(), emax.unwrap()))
    }
}

impl<T: PartialOrd + Copy> From<UiValue<T>> for LayoutField<T> {
    fn from(value: UiValue<T>) -> Self {
        LayoutField {
            value,
            min: UiValue::None,
            max: UiValue::None,
        }
    }
}

impl<T: PartialOrd + Clone> LayoutField<T> {
    pub fn is_set(&self) -> bool {
        return self.value.is_set()
    }

    pub fn is_none(&self) -> bool {
        return matches!(self.value, UiValue::None)
    }

    pub fn is_auto(&self) -> bool {
        return matches!(self.value, UiValue::Auto)
    }

    pub fn apply<F>(&self, value: T, elem: &dyn UiElement, map: F) -> T where F: Fn(&UiStyle) -> &Self {
        let min = self.min.resolve(elem.state().ctx.dpi, elem.state().parent.clone(), |s| &map(s).min);
        let max = self.min.resolve(elem.state().ctx.dpi, elem.state().parent.clone(), |s| &map(s).max);

        let mut ret = value;

        if min.is_some() {
            let min = min.unwrap();
            if ret < min {
                ret = min;
            }
        }

        if max.is_some() {
            let max = max.unwrap();
            if ret < max {
                ret = max;
            }
        }

        ret
    }
}

#[derive(Clone, Debug)]
pub struct Dimension<T: Num + Clone + Debug> {
    pub width: T,
    pub height: T,
}

impl<T: Num + Clone + Debug> Dimension<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

#[derive(Clone)]
pub struct Point<T: Num + Clone> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Clone> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct SideStyle {
    pub top: LayoutField<i32>,
    pub bottom: LayoutField<i32>,
    pub left: LayoutField<i32>,
    pub right: LayoutField<i32>,
}

impl SideStyle {
    pub fn all_i32(v: i32) -> Self {
        Self {
            top: UiValue::Just(v).into(),
            bottom: UiValue::Just(v).into(),
            left: UiValue::Just(v).into(),
            right: UiValue::Just(v).into(),
        }
    }

    pub fn all(v: UiValue<i32>) -> Self {
        Self {
            top: v.clone().into(),
            bottom: v.clone().into(),
            left: v.clone().into(),
            right: v.into(),
        }
    }

    pub fn set(&mut self, v: UiValue<i32>) {
        self.top = v.clone().into();
        self.bottom = v.clone().into();
        self.left = v.clone().into();
        self.right = v.into();
    }

    pub fn get(&self, elem: &dyn UiElement) -> [i32; 4] {
        let top = if self.top.is_set() {
            resolve!(elem, padding.top)
        } else {
            self.top.is_auto().yn(5, 0)
        };
        let bottom = if self.bottom.is_set() {
            resolve!(elem, padding.bottom)
        } else {
            self.bottom.is_auto().yn(5, 0)
        };
        let left = if self.left.is_set() {
            resolve!(elem, padding.left)
        } else {
            self.left.is_auto().yn(5, 0)
        };
        let right = if self.right.is_set() {
            resolve!(elem, padding.right)
        } else {
            self.left.is_auto().yn(5, 0)
        };
        [top, bottom, left, right]
    }
}

pub trait Resolve<T> {
    fn resolve<F>(&self, dpi: f32, parent: Option<Arc<RwLock<dyn UiElement>>>, map: F) -> Option<T>
        where
            F: Fn(&UiStyle) -> &Self;
}

#[derive(Clone, Default)]
pub enum UiValue<T: Clone + 'static> {
    #[default]
    None,
    Auto,
    Inherit,
    Clone(Arc<dyn UiElement>),
    Just(T),
    Measurement(Unit),
}

impl<T: Clone + 'static> Resolve<T> for UiValue<T> {
    fn resolve<F>(&self, dpi: f32, parent: Option<Arc<RwLock<dyn UiElement>>>, map: F) -> Option<T>
    where
        F: Fn(&UiStyle) -> &Self,
    {
        match self {
            UiValue::None => None,
            UiValue::Auto => None,
            UiValue::Inherit => {
                let lock = parent.clone().unwrap_or_else(no_parent);
                let guard = lock.read();
                map(guard.style()).resolve(
                    dpi,
                    Some(
                        parent
                            .unwrap_or_else(no_parent)
                            .read()
                            .state().parent.clone()
                            .unwrap_or_else(no_parent)
                            .clone(),
                    ),
                    map,
                )
            },
            UiValue::Clone(e) => map(e.style()).resolve(dpi, e.state().parent.clone(), map),
            UiValue::Just(v) => Some(v.clone()),
            UiValue::Measurement(u) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
                    unsafe {
                        let a = u.as_px(dpi);
                        Some(Unsafe::cast_ref::<i32, T>(&a).clone())
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl<T: Clone + 'static> UiValue<T> {
    pub fn is_set(&self) -> bool {
        !matches!(self, UiValue::None | UiValue::Auto)
    }
}

//impl<T: Clone + 'static> PartialEq for UiValue<T> {
//    fn eq(&self, other: &Self) -> bool {
//        matches!(self, other)
//    }
//}

fn no_parent<T>() -> T {
    panic!("Called Inherit on UiElement without parent")
}

impl Default for UiStyle {
    fn default() -> Self {
        Self {
            x: UiValue::Just(0).into(),
            y: UiValue::Just(0).into(),
            width: UiValue::Auto.into(),
            height: UiValue::Auto.into(),
            padding: SideStyle::all_i32(0),
            margin: SideStyle::all_i32(0),
            origin: UiValue::Just(Origin::BottomLeft),
            position: UiValue::Just(Position::Relative),
            rotation_origin: UiValue::Just(Origin::Center),
            rotation: UiValue::Just(0.0),
            direction: UiValue::Auto,
            child_align: UiValue::Auto,
            text: TextStyle::initial(),
            background: BackgroundInfo::default(),
        }
    }
}

pub(crate) struct ResCon {
    pub dpi: f32,
}

impl ResCon {
    pub(crate) fn set_dpi(&mut self, dpi: f32) {
        self.dpi = dpi;
    }
}

#[derive(Clone, Copy)]
pub enum Unit {
    Px(i32),         // px
    MM(f32),         // mm
    CM(f32),         // cm
    M(f32),          // m
    In(f32),         // in
    Twip(f32),       // twip
    Mil(f32),        // mil
    Point(f32),      // pt
    Pica(f32),       // pica
    Foot(f32),       // ft
    Yard(f32),       // yd
    Link(f32),       // lk
    Rod(f32),        // rd
    Chain(f32),      // ch
    Line(f32),       // ln
    BarleyCorn(f32), // bc
    Nail(f32),       // nl
    Finger(f32),     // fg
    Stick(f32),      // sk
    Palm(f32),       // pm
    Shaftment(f32),  // sf
    Span(f32),       // sp
    Quarter(f32),    // qr
    Pace(f32),       // pc
}

impl Unit {
    pub fn as_px(&self, dpi: f32) -> i32 {
        match self {
            Unit::Px(px) => *px,
            Unit::MM(value) => ((value / 25.4) * dpi) as i32,
            Unit::CM(value) => ((value / 2.54) * dpi) as i32,
            Unit::M(value) => (value * dpi) as i32,
            Unit::In(value) => (value * dpi) as i32,
            Unit::Twip(value) => ((value / 1440.0) * dpi) as i32,
            Unit::Mil(value) => ((value / 1000.0) * dpi) as i32,
            Unit::Point(value) => (value * (dpi / 72.0)) as i32,
            Unit::Pica(value) => (value * (dpi / 6.0)) as i32,
            Unit::Foot(value) => ((value * 12.0) * dpi) as i32,
            Unit::Yard(value) => ((value * 36.0) * dpi) as i32,
            Unit::Link(value) => ((value * 7.92) * dpi) as i32,
            Unit::Rod(value) => ((value * 198.0) * dpi) as i32,
            Unit::Chain(value) => ((value * 792.0) * dpi) as i32,
            Unit::Line(value) => ((value * 0.792) * dpi) as i32,
            Unit::BarleyCorn(value) => ((value * 0.125) * dpi) as i32,
            Unit::Nail(value) => ((value * 0.25) * dpi) as i32,
            Unit::Finger(value) => ((value * 0.375) * dpi) as i32,
            Unit::Stick(value) => ((value * 0.5) * dpi) as i32,
            Unit::Palm(value) => ((value * 3.0) * dpi) as i32,
            Unit::Shaftment(value) => ((value * 6.0) * dpi) as i32,
            Unit::Span(value) => ((value * 9.0) * dpi) as i32,
            Unit::Quarter(value) => ((value * 36.0) * dpi) as i32,
            Unit::Pace(value) => ((value * 30.0) * dpi) as i32,
        }
    }
}
