use super::visuals::VisualRules;

#[derive(Debug, Default)]
pub struct OuterBox {
    pub top_y: f64,
    pub bottom_y: f64,
    pub left_x: f64,
    pub right_x: f64,
}

#[derive(Debug, Default)]
pub struct InnerBox {
		pub x: f64,
		pub y: f64,
}

impl InnerBox {
    pub fn new(visual_rules: &VisualRules) -> InnerBox {
        InnerBox {
            x: 0.0,
            y: visual_rules.font_size,
        }
    }
}

impl OuterBox {
    pub fn new(visual_rules: &VisualRules) -> OuterBox {
        OuterBox {
            top_y: visual_rules.padding.top,
            bottom_y: visual_rules.padding.bottom,
            left_x: visual_rules.padding.left,
            right_x: visual_rules.padding.right,
        }
    }
}
