#[derive(Debug, Clone)]
pub struct Document {
    pub raw: String,
    pub blocks: Vec<Block>,
}

impl Document {
    pub fn new(raw: String, blocks: Vec<Block>) -> Self {
        Self { raw, blocks }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub kind: BlockKind,
    pub text: String,
    pub raw: String,
    pub index: usize,
}

impl Block {
    pub fn new(kind: BlockKind, text: String, raw: String, index: usize) -> Self {
        Self {
            kind,
            text,
            raw,
            index,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockKind {
    Heading { level: u8 },
    Paragraph,
    ListItem { ordered: bool, list_id: usize },
    CodeBlock { info: Option<String> },
    BlockQuote,
}

impl BlockKind {
    pub fn label(&self) -> &'static str {
        match self {
            BlockKind::Heading { .. } => "Heading",
            BlockKind::Paragraph => "Paragraph",
            BlockKind::ListItem { .. } => "List item",
            BlockKind::CodeBlock { .. } => "Code block",
            BlockKind::BlockQuote => "Block quote",
        }
    }
}
