use std::collections::BTreeMap;

use serde::*;
use uiua::primitive::Primitive;

fn main() {
    let mut map = BTreeMap::new();
    for prim in Primitive::non_deprecated() {
        if let Some(names) = prim.names() {
            map.insert(
                names.text,
                PrimData {
                    glyph: names.glyph,
                    description: prim
                        .doc()
                        .map(|doc| doc.short_text())
                        .unwrap_or_default()
                        .into(),
                },
            );
        }
    }
    let json = serde_json::to_string_pretty(&map).unwrap();
    std::fs::write("uiua_functions.json", json).unwrap();
}

#[derive(Serialize)]
struct PrimData {
    #[serde(skip_serializing_if = "Option::is_none")]
    glyph: Option<char>,
    description: String,
}
