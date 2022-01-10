pub enum ToolTyp {
    Button,
}

pub struct Tool {
    icon: &str,
    description: &str,
    typ: ToolTyp,
    is_selected: bool,
}

