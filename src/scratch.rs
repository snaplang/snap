use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

/// Helper function to serialize Option<String> as null instead of skipping
fn serialize_option_null<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(s) => serializer.serialize_some(s),
        None => serializer.serialize_none(),
    }
}

/// Root structure of a Scratch 3.0 project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchProject {
    pub targets: Vec<Target>,
    pub monitors: Vec<Monitor>,
    pub extensions: Vec<String>,
    pub meta: Meta,
}

impl ScratchProject {
    pub fn new() -> Self {
        ScratchProject {
            targets: Vec::new(),
            monitors: Vec::new(),
            extensions: Vec::new(),
            meta: Meta::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,
    pub broadcasts: HashMap<String, String>,
    pub blocks: HashMap<String, Block>,
    pub comments: HashMap<String, Comment>,
    pub current_costume: usize,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub layer_order: usize,
    pub volume: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_transparency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_style: Option<String>,
}

impl Target {
    pub fn new_stage() -> Self {
        Target {
            is_stage: true,
            name: "Stage".to_string(),
            variables: HashMap::new(),
            lists: HashMap::new(),
            broadcasts: HashMap::new(),
            blocks: HashMap::new(),
            comments: HashMap::new(),
            current_costume: 0,
            costumes: vec![Costume::default_backdrop()],
            sounds: Vec::new(),
            layer_order: 0,
            volume: 100.0,
            tempo: Some(60.0),
            video_transparency: Some(50.0),
            video_state: Some("on".to_string()),
            text_to_speech_language: None,
            visible: None,
            x: None,
            y: None,
            size: None,
            direction: None,
            draggable: None,
            rotation_style: None,
        }
    }

    pub fn new_sprite(name: &str, layer_order: usize) -> Self {
        Target {
            is_stage: false,
            name: name.to_string(),
            variables: HashMap::new(),
            lists: HashMap::new(),
            broadcasts: HashMap::new(),
            blocks: HashMap::new(),
            comments: HashMap::new(),
            current_costume: 0,
            costumes: vec![Costume::default_costume()],
            sounds: Vec::new(),
            layer_order,
            volume: 100.0,
            tempo: None,
            video_transparency: None,
            video_state: None,
            text_to_speech_language: None,
            visible: Some(true),
            x: Some(0.0),
            y: Some(0.0),
            size: Some(100.0),
            direction: Some(90.0),
            draggable: Some(false),
            rotation_style: Some("all around".to_string()),
        }
    }
}

/// Variable: [name, value]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Variable(pub (String, serde_json::Value));

impl Variable {
    pub fn new(name: &str, value: serde_json::Value) -> Self {
        Variable((name.to_string(), value))
    }
}

/// List: [name, [items]]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct List(pub (String, Vec<serde_json::Value>));

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub opcode: String,
    #[serde(serialize_with = "serialize_option_null")]
    pub next: Option<String>,
    #[serde(serialize_with = "serialize_option_null")]
    pub parent: Option<String>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    pub shadow: bool,
    pub top_level: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation: Option<Mutation>,
}

impl Block {
    pub fn new(opcode: &str) -> Self {
        Block {
            opcode: opcode.to_string(),
            next: None,
            parent: None,
            inputs: HashMap::new(),
            fields: HashMap::new(),
            shadow: false,
            top_level: false,
            x: None,
            y: None,
            mutation: None,
        }
    }

    #[allow(dead_code)] // Utility method for future use
    pub fn top_level(mut self, x: f64, y: f64) -> Self {
        self.top_level = true;
        self.x = Some(x);
        self.y = Some(y);
        self
    }
}

/// Input values in Scratch blocks
/// Format: [shadow_type, value] or [shadow_type, value, obscured_shadow]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Input(pub Vec<serde_json::Value>);

impl Input {
    /// Create a simple literal input (number or string)
    /// Format: [1, [type, value]]
    pub fn literal(input_type: u8, value: serde_json::Value) -> Self {
        Input(vec![
            serde_json::json!(1),
            serde_json::json!([input_type, value]),
        ])
    }

    /// Create a block reference input
    /// Format: [2, block_id]
    pub fn block(block_id: &str) -> Self {
        Input(vec![serde_json::json!(2), serde_json::json!(block_id)])
    }

    /// Create a block reference with shadow
    /// Format: [3, block_id, [type, value]]
    #[allow(dead_code)] // Utility method for future use
    pub fn block_with_shadow(
        block_id: &str,
        shadow_type: u8,
        shadow_value: serde_json::Value,
    ) -> Self {
        Input(vec![
            serde_json::json!(3),
            serde_json::json!(block_id),
            serde_json::json!([shadow_type, shadow_value]),
        ])
    }

    /// Create a substack input (for C-blocks like forever, if, etc.)
    /// Format: [2, block_id]
    pub fn substack(block_id: Option<&str>) -> Self {
        match block_id {
            Some(id) => Input(vec![serde_json::json!(2), serde_json::json!(id)]),
            None => Input(vec![serde_json::json!(1), serde_json::Value::Null]),
        }
    }

    /// Create a variable input
    /// Format: [3, [12, var_name, var_id], [type, default]]
    pub fn variable(var_name: &str, var_id: &str) -> Self {
        Input(vec![
            serde_json::json!(3),
            serde_json::json!([12, var_name, var_id]),
            serde_json::json!([10, ""]),
        ])
    }
}

/// Field values in Scratch blocks
/// Format: [value, id] where id is optional
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Field(pub Vec<serde_json::Value>);

impl Field {
    pub fn new(value: &str, id: Option<&str>) -> Self {
        Field(vec![
            serde_json::json!(value),
            match id {
                Some(i) => serde_json::json!(i),
                None => serde_json::Value::Null,
            },
        ])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub tag_name: String,
    pub children: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proccode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argumentids: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argumentnames: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argumentdefaults: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hasnext: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    pub asset_id: String,
    pub name: String,
    pub md5ext: String,
    pub data_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitmap_resolution: Option<u32>,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,
}

impl Costume {
    pub fn default_backdrop() -> Self {
        Costume {
            asset_id: "cd21514d0531fdffb22204e0ec5ed84a".to_string(),
            name: "backdrop1".to_string(),
            md5ext: "cd21514d0531fdffb22204e0ec5ed84a.svg".to_string(),
            data_format: "svg".to_string(),
            bitmap_resolution: None,
            rotation_center_x: 240.0,
            rotation_center_y: 180.0,
        }
    }

    pub fn default_costume() -> Self {
        Costume {
            asset_id: "bcf454acf82e4504149f7ffe07081571".to_string(),
            name: "costume1".to_string(),
            md5ext: "bcf454acf82e4504149f7ffe07081571.svg".to_string(),
            data_format: "svg".to_string(),
            bitmap_resolution: None,
            rotation_center_x: 48.0,
            rotation_center_y: 50.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    pub asset_id: String,
    pub name: String,
    pub data_format: String,
    pub rate: u32,
    pub sample_count: u32,
    pub md5ext: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub minimized: bool,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub mode: String,
    pub opcode: String,
    pub params: HashMap<String, String>,
    #[serde(rename = "spriteName")]
    pub sprite_name: Option<String>,
    pub value: serde_json::Value,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub visible: bool,
    #[serde(rename = "sliderMin")]
    pub slider_min: f64,
    #[serde(rename = "sliderMax")]
    pub slider_max: f64,
    #[serde(rename = "isDiscrete")]
    pub is_discrete: bool,
}

impl Monitor {
    /// Create a variable monitor
    pub fn variable(id: &str, name: &str, value: serde_json::Value, x: f64, y: f64) -> Self {
        let mut params = HashMap::new();
        params.insert("VARIABLE".to_string(), name.to_string());

        Monitor {
            id: id.to_string(),
            mode: "default".to_string(),
            opcode: "data_variable".to_string(),
            params,
            sprite_name: None,
            value,
            width: 0.0,
            height: 0.0,
            x,
            y,
            visible: false,
            slider_min: 0.0,
            slider_max: 100.0,
            is_discrete: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub semver: String,
    pub vm: String,
    pub agent: String,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            semver: "3.0.0".to_string(),
            vm: "12.1.3".to_string(),
            agent: format!("snap-compiler/{}", env!("CARGO_PKG_VERSION")).to_string(),
        }
    }
}

// Input type constants (used in Input literals)
#[allow(dead_code)] // Constants for Scratch input types - kept for completeness
pub mod input_types {
    pub const NUMBER: u8 = 4;
    pub const POSITIVE_NUMBER: u8 = 5;
    pub const POSITIVE_INT: u8 = 6;
    pub const INTEGER: u8 = 7;
    pub const ANGLE: u8 = 8;
    pub const COLOR: u8 = 9;
    pub const STRING: u8 = 10;
    pub const BROADCAST: u8 = 11;
    pub const VARIABLE: u8 = 12;
    pub const LIST: u8 = 13;
}
