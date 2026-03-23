#[derive(Debug,PartialEq)]
pub enum LabelType {
    Select,
    Text,
    Line,
}

#[derive(Debug,PartialEq)]
pub enum LabelStyle {
    Border,
    DobleBorder,
    BottomBorder,
    Edges,
    Text,
}

#[derive(Debug)]
pub struct Label {
    pub text: String,
    pub label_type: LabelType,
    pub style: LabelStyle,
    pub pos_x: u16,
    pub pos_y: u16,
}

