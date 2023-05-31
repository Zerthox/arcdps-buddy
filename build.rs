#[path = "src/data/structs.rs"]
mod structs;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};
use structs::SkillDef;

fn main() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let in_dir = PathBuf::from(manifest).join("src/data");

    let files = fs::read_dir(in_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("json" | "yml" | "yaml")
            )
        });

    let data = files.flat_map(|file| {
        println!("cargo:rerun-if-changes={}", file.display());
        serde_yaml::from_reader::<_, Vec<SkillDef>>(File::open(file).unwrap()).unwrap()
    });

    let contents = data.map(
        |SkillDef {
             id,
             hit_id,
             hits,
             expected,
         }| {
            let hit_id = quote_option(hit_id);
            let hits = quote_option(hits);
            let expected = quote_option(expected);
            quote! {
                SkillDef {
                    id: #id,
                    hit_id: #hit_id,
                    hits: #hits,
                    expected: #expected,
                }
            }
        },
    );

    let result = quote! { [ #(#contents),* ] };

    fs::write(PathBuf::from(out_dir).join("skills.rs"), result.to_string()).unwrap();
}

fn quote_option(option: Option<impl ToTokens>) -> TokenStream {
    option
        .map(|value| quote! { Some(#value) })
        .unwrap_or(quote! { None })
}
