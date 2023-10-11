use serde::*;
use uiua::primitive::Primitive;

fn main() {
    let mut prims = Vec::new();
    for prim in Primitive::non_deprecated() {
        if let Some(names) = prim.names() {
            prims.push(PrimData {
                name: names.text.into(),
                glyph: names.glyph,
                description: prim
                    .doc()
                    .map(|doc| doc.short_text())
                    .unwrap_or_default()
                    .into(),
            });
        }
    }
    let json = serde_json::to_string_pretty(&prims).unwrap();
    std::fs::write("uiua_functions.json", json).unwrap();
}

#[derive(Serialize)]
struct PrimData {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    glyph: Option<char>,
    description: String,
}
