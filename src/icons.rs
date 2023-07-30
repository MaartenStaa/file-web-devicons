use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub struct Icon {
    pub icon: &'static str,
    pub color_red: u8,
    pub color_green: u8,
    pub color_blue: u8,
}

impl Icon {
    fn new(icon: &'static str, color_hex: &str) -> Icon {
        let color = u32::from_str_radix(color_hex, 16).unwrap();
        Icon {
            icon,
            color_red: ((color >> 16) & 0xFF) as u8,
            color_green: ((color >> 8) & 0xFF) as u8,
            color_blue: (color & 0xFF) as u8,
        }
    }
}

// Source for all the icon definitions below:
// https://github.com/nvim-tree/nvim-web-devicons/blob/master/lua/nvim-web-devicons.lua

pub static DEFAULT_ICON: Lazy<Icon> = Lazy::new(|| Icon::new("", "6d8086"));

pub static ICONS_BY_FILENAME: Lazy<Mutex<HashMap<&str, Icon>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(".env", Icon::new("", "faf743"));
    m.insert("build", Icon::new("", "89e051"));
    m.insert("dockerfile", Icon::new("󰡨", "458ee6"));
    m.insert("favicon.ico", Icon::new("", "cbcb41"));
    m.insert("gnumakefile", Icon::new("", "6d8086"));
    m.insert("gruntfile", Icon::new("", "e37933"));
    m.insert("gulpfile", Icon::new("", "cc3e44"));
    m.insert("makefile", Icon::new("", "6d8086"));
    m.insert("mix.lock", Icon::new("", "a074c4"));
    m.insert("node_modules", Icon::new("", "E8274B"));
    m.insert("package-lock.json", Icon::new("", "7a0d21"));
    m.insert("package.json", Icon::new("", "e8274b"));
    m.insert("procfile", Icon::new("", "a074c4"));
    m.insert("rakefile", Icon::new("", "701516"));
    m.insert("unlicense", Icon::new("", "d0bf41"));
    m.insert("webpack", Icon::new("󰜫", "519aba"));
    m.insert("workspace", Icon::new("", "89e051"));
    Mutex::new(m)
});

pub static ICONS_BY_FILE_EXTENSION: Lazy<Mutex<HashMap<&str, Icon>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("epp", Icon::new("", "FFA61A"));
    m.insert("erb", Icon::new("", "701516"));
    m.insert("erl", Icon::new("", "B83998"));
    m.insert("ex", Icon::new("", "a074c4"));
    m.insert("exs", Icon::new("", "a074c4"));
    m.insert("f#", Icon::new("", "519aba"));
    m.insert("f90", Icon::new("󱈚", "734f96"));
    m.insert("fish", Icon::new("", "4d5a5e"));
    m.insert("fnl", Icon::new("🌜", "fff3d7"));
    m.insert("fs", Icon::new("", "519aba"));
    m.insert("fsi", Icon::new("", "519aba"));
    m.insert("fsscript", Icon::new("", "519aba"));
    m.insert("fsx", Icon::new("", "519aba"));
    m.insert("gd", Icon::new("", "6d8086"));
    m.insert("gemspec", Icon::new("", "701516"));
    m.insert("gif", Icon::new("", "a074c4"));
    m.insert("git", Icon::new("", "F14C28"));
    m.insert("glb", Icon::new("", "FFB13B"));
    m.insert("gnumakefile", Icon::new("", "6d8086"));
    m.insert("go", Icon::new("", "519aba"));
    m.insert("godot", Icon::new("", "6d8086"));
    m.insert("gql", Icon::new("", "e535ab"));
    m.insert("graphql", Icon::new("", "e535ab"));
    m.insert("h", Icon::new("", "a074c4"));
    m.insert("haml", Icon::new("", "eaeae1"));
    m.insert("hbs", Icon::new("", "f0772b"));
    m.insert("heex", Icon::new("", "a074c4"));
    m.insert("hh", Icon::new("", "a074c4"));
    m.insert("hpp", Icon::new("", "a074c4"));
    m.insert("hrl", Icon::new("", "B83998"));
    m.insert("hs", Icon::new("", "a074c4"));
    m.insert("htm", Icon::new("", "e34c26"));
    m.insert("html", Icon::new("", "e44d26"));
    m.insert("hxx", Icon::new("", "a074c4"));
    m.insert("ico", Icon::new("", "cbcb41"));
    m.insert("import", Icon::new("", "ECECEC"));
    m.insert("ini", Icon::new("", "6d8086"));
    m.insert("java", Icon::new("", "cc3e44"));
    m.insert("jl", Icon::new("", "a270ba"));
    m.insert("jpeg", Icon::new("", "a074c4"));
    m.insert("jpg", Icon::new("", "a074c4"));
    m.insert("js", Icon::new("", "cbcb41"));
    m.insert("json", Icon::new("", "cbcb41"));
    m.insert("json5", Icon::new("", "cbcb41"));
    m.insert("jsx", Icon::new("", "20c2e3"));
    m.insert("ksh", Icon::new("", "4d5a5e"));
    m.insert("kt", Icon::new("", "7F52FF"));
    m.insert("kts", Icon::new("", "7F52FF"));
    m.insert("leex", Icon::new("", "a074c4"));
    m.insert("less", Icon::new("", "563d7c"));
    m.insert("lhs", Icon::new("", "a074c4"));
    m.insert("license", Icon::new("", "cbcb41"));
    m.insert("liquid", Icon::new("", "95BF47"));
    m.insert("lock", Icon::new("", "bbbbbb"));
    m.insert("log", Icon::new("󰌱", "ffffff"));
    m.insert("lua", Icon::new("", "51a0cf"));
    m.insert("luau", Icon::new("", "51a0cf"));
    m.insert("makefile", Icon::new("", "6d8086"));
    m.insert("markdown", Icon::new("", "519aba"));
    m.insert("material", Icon::new("󰔉", "B83998"));
    m.insert("md", Icon::new("", "ffffff"));
    m.insert("mdx", Icon::new("", "519aba"));
    m.insert("mint", Icon::new("󰌪", "87c095"));
    m.insert("mjs", Icon::new("", "f1e05a"));
    m.insert("mk", Icon::new("", "6d8086"));
    m.insert("ml", Icon::new("λ", "e37933"));
    m.insert("mli", Icon::new("λ", "e37933"));
    m.insert("mo", Icon::new("∞", "9772FB"));
    m.insert("mustache", Icon::new("", "e37933"));
    m.insert("nim", Icon::new("", "f3d400"));
    m.insert("nix", Icon::new("", "7ebae4"));
    m.insert("opus", Icon::new("󰈣", "F88A02"));
    m.insert("org", Icon::new("", "77AA99"));
    m.insert("otf", Icon::new("", "ECECEC"));
    m.insert("pck", Icon::new("", "6d8086"));
    m.insert("pdf", Icon::new("", "b30b00"));
    m.insert("php", Icon::new("", "a074c4"));
    m.insert("pl", Icon::new("", "519aba"));
    m.insert("pm", Icon::new("", "519aba"));
    m.insert("png", Icon::new("", "a074c4"));
    m.insert("pp", Icon::new("", "FFA61A"));
    m.insert("ppt", Icon::new("󰈧", "cb4a32"));
    m.insert("prisma", Icon::new("󰔶", "ffffff"));
    m.insert("pro", Icon::new("", "e4b854"));
    m.insert("ps1", Icon::new("󰨊", "4273ca"));
    m.insert("psb", Icon::new("", "519aba"));
    m.insert("psd", Icon::new("", "519aba"));
    m.insert("psd1", Icon::new("󰨊", "6975c4"));
    m.insert("psm1", Icon::new("󰨊", "6975c4"));
    m.insert("py", Icon::new("", "ffbc03"));
    m.insert("pyc", Icon::new("", "ffe291"));
    m.insert("pyd", Icon::new("", "ffe291"));
    m.insert("pyo", Icon::new("", "ffe291"));
    m.insert("query", Icon::new("", "90a850"));
    m.insert("r", Icon::new("󰟔", "358a5b"));
    m.insert("rake", Icon::new("", "701516"));
    m.insert("rb", Icon::new("", "701516"));
    m.insert("res", Icon::new("", "cc3e44"));
    m.insert("resi", Icon::new("", "f55385"));
    m.insert("rlib", Icon::new("", "dea584"));
    m.insert("rmd", Icon::new("", "519aba"));
    m.insert("rproj", Icon::new("󰗆", "358a5b"));
    m.insert("rs", Icon::new("", "dea584"));
    m.insert("rss", Icon::new("", "FB9D3B"));
    m.insert("sass", Icon::new("", "f55385"));
    m.insert("sbt", Icon::new("", "cc3e44"));
    m.insert("scala", Icon::new("", "cc3e44"));
    m.insert("scm", Icon::new("󰘧", "000000"));
    m.insert("scss", Icon::new("", "f55385"));
    m.insert("sh", Icon::new("", "4d5a5e"));
    m.insert("sig", Icon::new("λ", "e37933"));
    m.insert("slim", Icon::new("", "e34c26"));
    m.insert("sln", Icon::new("", "854CC7"));
    m.insert("sml", Icon::new("λ", "e37933"));
    m.insert("sol", Icon::new("󰞻", "519aba"));
    m.insert("spec.js", Icon::new("", "cbcb41"));
    m.insert("spec.jsx", Icon::new("", "20c2e3"));
    m.insert("spec.ts", Icon::new("", "519aba"));
    m.insert("spec.tsx", Icon::new("", "1354bf"));
    m.insert("sql", Icon::new("", "dad8d8"));
    m.insert("sqlite", Icon::new("", "dad8d8"));
    m.insert("sqlite3", Icon::new("", "dad8d8"));
    m.insert("styl", Icon::new("", "8dc149"));
    m.insert("sublime", Icon::new("", "e37933"));
    m.insert("suo", Icon::new("", "854CC7"));
    m.insert("sv", Icon::new("󰍛", "019833"));
    m.insert("svelte", Icon::new("", "ff3e00"));
    m.insert("svg", Icon::new("󰜡", "FFB13B"));
    m.insert("svh", Icon::new("󰍛", "019833"));
    m.insert("swift", Icon::new("", "e37933"));
    m.insert("t", Icon::new("", "519aba"));
    m.insert("tbc", Icon::new("󰛓", "1e5cb3"));
    m.insert("tcl", Icon::new("󰛓", "1e5cb3"));
    m.insert("terminal", Icon::new("", "31B53E"));
    m.insert("test.js", Icon::new("", "cbcb41"));
    m.insert("test.jsx", Icon::new("", "20c2e3"));
    m.insert("test.ts", Icon::new("", "519aba"));
    m.insert("test.tsx", Icon::new("", "1354bf"));
    m.insert("tex", Icon::new("󰙩", "3D6117"));
    m.insert("tf", Icon::new("", "5F43E9"));
    m.insert("tfvars", Icon::new("", "5F43E9"));
    m.insert("toml", Icon::new("", "6d8086"));
    m.insert("tres", Icon::new("", "cbcb41"));
    m.insert("ts", Icon::new("", "519aba"));
    m.insert("tscn", Icon::new("󰎁", "a074c4"));
    m.insert("tsx", Icon::new("", "1354bf"));
    m.insert("twig", Icon::new("", "8dc149"));
    m.insert("txt", Icon::new("󰈙", "89e051"));
    m.insert("v", Icon::new("󰍛", "019833"));
    m.insert("vala", Icon::new("", "7239b3"));
    m.insert("vh", Icon::new("󰍛", "019833"));
    m.insert("vhd", Icon::new("󰍛", "019833"));
    m.insert("vhdl", Icon::new("󰍛", "019833"));
    m.insert("vim", Icon::new("", "019833"));
    m.insert("vue", Icon::new("", "8dc149"));
    m.insert("wasm", Icon::new("", "5c4cdb"));
    m.insert("webmanifest", Icon::new("", "f1e05a"));
    m.insert("webp", Icon::new("", "a074c4"));
    m.insert("webpack", Icon::new("󰜫", "519aba"));
    m.insert("xcplayground", Icon::new("", "e37933"));
    m.insert("xls", Icon::new("󰈛", "207245"));
    m.insert("xlsx", Icon::new("󰈛", "207245"));
    m.insert("xml", Icon::new("󰗀", "e37933"));
    m.insert("xul", Icon::new("", "e37933"));
    m.insert("yaml", Icon::new("", "6d8086"));
    m.insert("yml", Icon::new("", "6d8086"));
    m.insert("zig", Icon::new("", "f69a1b"));
    m.insert("zsh", Icon::new("", "89e051"));
    Mutex::new(m)
});
