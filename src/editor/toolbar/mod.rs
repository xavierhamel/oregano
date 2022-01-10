mod tool;

pub struct ToolBar {
    tools: Vec<tool::Tool>,
}

impl ToolBar {
    pub fn new() -> Self {
        let mut tools = Vec::new();
        
        tools.push(
            Tool {
                icon: 'C',

            }
        )
    }
}
