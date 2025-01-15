//! This script is used to generate the C# bindings for the `RawModuleDef` type.
//! Run `cargo run --example regen-csharp-moduledef` to update C# bindings whenever the module definition changes.

use fs_err as fs;
use regex::Regex;
use spacetimedb_cli::generate::{generate, Language};
use spacetimedb_lib::{RawModuleDef, RawModuleDefV8};
use std::path::Path;
use std::sync::OnceLock;

macro_rules! regex_replace {
    ($value:expr, $re:expr, $replace:expr) => {{
        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
            .replace_all($value, $replace)
    }};
}

fn main() -> anyhow::Result<()> {
    let module = RawModuleDefV8::with_builder(|module| {
        module.add_type::<RawModuleDef>();
    });

    let mut results = generate(
        RawModuleDef::V8BackCompat(module),
        Language::Csharp,
        "SpacetimeDB.Internal",
    )?;

    // Someday we might replace custom BSATN types with autogenerated ones as well,
    // but for now they're not very large and our copies are somewhat more optimised.
    //
    // Ignore those types and replace their references with our own with plain old regexes.

    results.retain(|(filename, _)| {
        !(matches!(filename.as_str(), "AlgebraicType.cs" | "MapType.cs")
            || filename.starts_with("_Globals")
            || filename.starts_with("SumType")
            || filename.starts_with("ProductType"))
    });

    for (_, code) in &mut results {
        let res = regex_replace!(
            code,
            r"\b(SpacetimeDB\.)Internal(\.(Algebraic|Map)Type)\b",
            "${1}BSATN${2}"
        );
        let res = regex_replace!(
            &res,
            r"\b(SpacetimeDB\.)Internal\.(ProductTypeElement|SumTypeVariant)\b",
            "${1}BSATN.AggregateElement"
        );
        let res = regex_replace!(
            &res,
            r"\b(SpacetimeDB\.)Internal(\.(Product|Sum)Type)\b",
            "List<${1}BSATN.AggregateElement>"
        );
        *code = res.into_owned();
    }

    let dir = &Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../bindings-csharp/Runtime/Internal/Autogen"
    ))
    .canonicalize()?;

    fs::remove_dir_all(dir)?;
    fs::create_dir(dir)?;

    for (file, content) in results {
        fs::write(dir.join(file), content)?;
    }

    Ok(())
}
