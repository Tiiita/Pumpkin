use std::collections::HashMap;
use std::sync::LazyLock;

use pumpkin_util::text::TextComponent;
use serde::Deserialize;

const ITEMS_JSON: &str = include_str!("../../../assets/items.json");

pub static ITEMS: LazyLock<HashMap<String, Item>> = LazyLock::new(|| {
    serde_json::from_str(ITEMS_JSON).expect("Could not parse items.json registry.")
});

pub static ITEMS_REGISTRY_NAME_BY_ID: LazyLock<HashMap<u16, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for item in ITEMS.clone() {
        map.insert(item.1.id, item.0.clone());
    }
    map
});

pub fn get_item(name: &str) -> Option<&Item> {
    ITEMS.get(&name.replace("minecraft:", ""))
}

pub fn get_name_by_id<'a>(item_id: u16) -> Option<&'a String> {
    ITEMS_REGISTRY_NAME_BY_ID.get(&item_id)
}

pub fn get_item_by_id<'a>(item_id: u16) -> Option<&'a Item> {
    ITEMS.values().find(|&item| item.id == item_id)
}

pub fn get_spawn_egg(item_id: u16) -> Option<String> {
    if let Some(item_name) = ITEMS_REGISTRY_NAME_BY_ID.get(&item_id) {
        if item_name.ends_with("_spawn_egg") {
            if let Some(res) = item_name.strip_suffix("_spawn_egg") {
                return Some(res.to_owned());
            }
        }
    };
    None
}

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    pub id: u16,
    pub components: ItemComponents,
}

impl Item {
    pub fn translated_name(&self) -> TextComponent {
        serde_json::from_str(&self.components.item_name).expect("Could not parse item name.")
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ItemComponents {
    #[serde(rename = "minecraft:item_name")]
    pub item_name: String,
    #[serde(rename = "minecraft:max_stack_size")]
    pub max_stack_size: u8,
    #[serde(rename = "minecraft:jukebox_playable")]
    pub jukebox_playable: Option<JukeboxPlayable>,
    #[serde(rename = "minecraft:damage")]
    pub damage: Option<u16>,
    #[serde(rename = "minecraft:max_damage")]
    pub max_damage: Option<u16>,
    #[serde(rename = "minecraft:attribute_modifiers")]
    pub attribute_modifiers: Option<AttributeModifiers>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JukeboxPlayable {
    pub song: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AttributeModifiers {
    pub modifiers: Vec<Modifier>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Modifier {
    pub r#type: String,
    pub id: String,
    pub amount: f64,
    pub operation: Operation,
    // TODO: Make this an enum
    pub slot: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}
