use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct Icon {
    pub icon: &'static str,
    pub color_red: u8,
    pub color_green: u8,
    pub color_blue: u8,
}

impl Icon {
    const fn new(icon: &'static str, color: u32) -> Icon {
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
// Use `scripts/update-icons.sh` to update the icons.

// BEGIN GENERATED CODE
pub static DEFAULT_ICON: Lazy<Icon> = Lazy::new(|| Icon::new("", 0x6D8086));

pub static ICONS_BY_FILENAME: Lazy<HashMap<&str, Icon>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(".babelrc", Icon::new("", 0xCBCB41));
    m.insert(".bash_profile", Icon::new("", 0x89E051));
    m.insert(".bashrc", Icon::new("", 0x89E051));
    m.insert(".dockerignore", Icon::new("󰡨", 0x458EE6));
    m.insert(".ds_store", Icon::new("", 0x41535B));
    m.insert(".editorconfig", Icon::new("", 0xFFFFFF));
    m.insert(".env", Icon::new("", 0xFAF743));
    m.insert(".eslintrc", Icon::new("", 0x4B32C3));
    m.insert(".gitattributes", Icon::new("", 0x41535B));
    m.insert(".gitconfig", Icon::new("", 0x41535B));
    m.insert(".gitignore", Icon::new("", 0x41535B));
    m.insert(".gitlab-ci.yml", Icon::new("", 0xE24329));
    m.insert(".gitmodules", Icon::new("", 0x41535B));
    m.insert(".gvimrc", Icon::new("", 0x019833));
    m.insert(".npmignore", Icon::new("", 0xE8274B));
    m.insert(".npmrc", Icon::new("", 0xE8274B));
    m.insert(".settings.json", Icon::new("", 0x854CC7));
    m.insert(".vimrc", Icon::new("", 0x019833));
    m.insert(".zprofile", Icon::new("", 0x89E051));
    m.insert(".zshenv", Icon::new("", 0x89E051));
    m.insert(".zshrc", Icon::new("", 0x89E051));
    m.insert("_gvimrc", Icon::new("", 0x019833));
    m.insert("_vimrc", Icon::new("", 0x019833));
    m.insert("brewfile", Icon::new("", 0x701516));
    m.insert("build", Icon::new("", 0x89E051));
    m.insert("cmakelists.txt", Icon::new("", 0x6D8086));
    m.insert("commit_editmsg", Icon::new("", 0x41535B));
    m.insert("containerfile", Icon::new("󰡨", 0x458EE6));
    m.insert("copying", Icon::new("", 0xCBCB41));
    m.insert("copying.lesser", Icon::new("", 0xCBCB41));
    m.insert("docker-compose.yaml", Icon::new("󰡨", 0x458EE6));
    m.insert("docker-compose.yml", Icon::new("󰡨", 0x458EE6));
    m.insert("dockerfile", Icon::new("󰡨", 0x458EE6));
    m.insert("favicon.ico", Icon::new("", 0xCBCB41));
    m.insert("gemfile$", Icon::new("", 0x701516));
    m.insert("gnumakefile", Icon::new("", 0x6D8086));
    m.insert("gruntfile", Icon::new("", 0xE37933));
    m.insert("gulpfile", Icon::new("", 0xCC3E44));
    m.insert("license", Icon::new("", 0xD0BF41));
    m.insert("makefile", Icon::new("", 0x6D8086));
    m.insert("mix.lock", Icon::new("", 0xA074C4));
    m.insert("node_modules", Icon::new("", 0xE8274B));
    m.insert("package-lock.json", Icon::new("", 0x7A0D21));
    m.insert("package.json", Icon::new("", 0xE8274B));
    m.insert("procfile", Icon::new("", 0xA074C4));
    m.insert("r", Icon::new("󰟔", 0x358A5B));
    m.insert("rakefile", Icon::new("", 0x701516));
    m.insert("rmd", Icon::new("", 0x519ABA));
    m.insert("svelte.config.js", Icon::new("", 0xFF3E00));
    m.insert("unlicense", Icon::new("", 0xD0BF41));
    m.insert("vagrantfile$", Icon::new("", 0x1563FF));
    m.insert("webpack", Icon::new("󰜫", 0x519ABA));
    m.insert("workspace", Icon::new("", 0x89E051));
    m
});

pub static ICONS_BY_FILE_EXTENSION: Lazy<HashMap<&str, Icon>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("ai", Icon::new("", 0xCBCB41));
    m.insert("awk", Icon::new("", 0x4D5A5E));
    m.insert("bash", Icon::new("", 0x89E051));
    m.insert("bat", Icon::new("", 0xC1F12E));
    m.insert("bazel", Icon::new("", 0x89E051));
    m.insert("bmp", Icon::new("", 0xA074C4));
    m.insert("bzl", Icon::new("", 0x89E051));
    m.insert("c", Icon::new("", 0x599EFF));
    m.insert("c++", Icon::new("", 0xF34B7D));
    m.insert("cbl", Icon::new("⚙", 0x005CA5));
    m.insert("cc", Icon::new("", 0xF34B7D));
    m.insert("cfg", Icon::new("", 0xECECEC));
    m.insert("cjs", Icon::new("", 0xCBCB41));
    m.insert("clj", Icon::new("", 0x8DC149));
    m.insert("cljc", Icon::new("", 0x8DC149));
    m.insert("cljd", Icon::new("", 0x519ABA));
    m.insert("cljs", Icon::new("", 0x519ABA));
    m.insert("cmake", Icon::new("", 0x6D8086));
    m.insert("cob", Icon::new("⚙", 0x005CA5));
    m.insert("cobol", Icon::new("⚙", 0x005CA5));
    m.insert("coffee", Icon::new("", 0xCBCB41));
    m.insert("conf", Icon::new("", 0x6D8086));
    m.insert("config.ru", Icon::new("", 0x701516));
    m.insert("cp", Icon::new("", 0x519ABA));
    m.insert("cpp", Icon::new("", 0x519ABA));
    m.insert("cpy", Icon::new("⚙", 0x005CA5));
    m.insert("cr", Icon::new("", 0xC8C8C8));
    m.insert("cs", Icon::new("󰌛", 0x596706));
    m.insert("csh", Icon::new("", 0x4D5A5E));
    m.insert("cson", Icon::new("", 0xCBCB41));
    m.insert("css", Icon::new("", 0x42A5F5));
    m.insert("csv", Icon::new("󰈙", 0x89E051));
    m.insert("cxx", Icon::new("", 0x519ABA));
    m.insert("d", Icon::new("", 0x427819));
    m.insert("dart", Icon::new("", 0x03589C));
    m.insert("db", Icon::new("", 0xDAD8D8));
    m.insert("desktop", Icon::new("", 0x563D7C));
    m.insert("diff", Icon::new("", 0x41535B));
    m.insert("doc", Icon::new("󰈬", 0x185ABD));
    m.insert("docx", Icon::new("󰈬", 0x185ABD));
    m.insert("drl", Icon::new("", 0xFFAFAF));
    m.insert("dropbox", Icon::new("", 0x0061FE));
    m.insert("dump", Icon::new("", 0xDAD8D8));
    m.insert("edn", Icon::new("", 0x519ABA));
    m.insert("eex", Icon::new("", 0xA074C4));
    m.insert("ejs", Icon::new("", 0xCBCB41));
    m.insert("elm", Icon::new("", 0x519ABA));
    m.insert("epp", Icon::new("", 0xFFA61A));
    m.insert("erb", Icon::new("", 0x701516));
    m.insert("erl", Icon::new("", 0xB83998));
    m.insert("ex", Icon::new("", 0xA074C4));
    m.insert("exs", Icon::new("", 0xA074C4));
    m.insert("f#", Icon::new("", 0x519ABA));
    m.insert("f90", Icon::new("󱈚", 0x734F96));
    m.insert("fish", Icon::new("", 0x4D5A5E));
    m.insert("fnl", Icon::new("🌜", 0xFFF3D7));
    m.insert("fs", Icon::new("", 0x519ABA));
    m.insert("fsi", Icon::new("", 0x519ABA));
    m.insert("fsscript", Icon::new("", 0x519ABA));
    m.insert("fsx", Icon::new("", 0x519ABA));
    m.insert("gd", Icon::new("", 0x6D8086));
    m.insert("gemspec", Icon::new("", 0x701516));
    m.insert("gif", Icon::new("", 0xA074C4));
    m.insert("git", Icon::new("", 0xF14C28));
    m.insert("glb", Icon::new("", 0xFFB13B));
    m.insert("gnumakefile", Icon::new("", 0x6D8086));
    m.insert("go", Icon::new("", 0x519ABA));
    m.insert("godot", Icon::new("", 0x6D8086));
    m.insert("gql", Icon::new("", 0xE535AB));
    m.insert("graphql", Icon::new("", 0xE535AB));
    m.insert("h", Icon::new("", 0xA074C4));
    m.insert("haml", Icon::new("", 0xEAEAE1));
    m.insert("hbs", Icon::new("", 0xF0772B));
    m.insert("heex", Icon::new("", 0xA074C4));
    m.insert("hh", Icon::new("", 0xA074C4));
    m.insert("hpp", Icon::new("", 0xA074C4));
    m.insert("hrl", Icon::new("", 0xB83998));
    m.insert("hs", Icon::new("", 0xA074C4));
    m.insert("htm", Icon::new("", 0xE34C26));
    m.insert("html", Icon::new("", 0xE44D26));
    m.insert("hxx", Icon::new("", 0xA074C4));
    m.insert("ico", Icon::new("", 0xCBCB41));
    m.insert("import", Icon::new("", 0xECECEC));
    m.insert("ini", Icon::new("", 0x6D8086));
    m.insert("java", Icon::new("", 0xCC3E44));
    m.insert("jl", Icon::new("", 0xA270BA));
    m.insert("jpeg", Icon::new("", 0xA074C4));
    m.insert("jpg", Icon::new("", 0xA074C4));
    m.insert("js", Icon::new("", 0xCBCB41));
    m.insert("json", Icon::new("", 0xCBCB41));
    m.insert("json5", Icon::new("", 0xCBCB41));
    m.insert("jsonc", Icon::new("", 0xCBCB41));
    m.insert("jsx", Icon::new("", 0x20C2E3));
    m.insert("ksh", Icon::new("", 0x4D5A5E));
    m.insert("kt", Icon::new("", 0x7F52FF));
    m.insert("kts", Icon::new("", 0x7F52FF));
    m.insert("leex", Icon::new("", 0xA074C4));
    m.insert("less", Icon::new("", 0x563D7C));
    m.insert("lhs", Icon::new("", 0xA074C4));
    m.insert("license", Icon::new("", 0xCBCB41));
    m.insert("liquid", Icon::new("", 0x95BF47));
    m.insert("lock", Icon::new("", 0xBBBBBB));
    m.insert("log", Icon::new("󰌱", 0xFFFFFF));
    m.insert("lua", Icon::new("", 0x51A0CF));
    m.insert("luau", Icon::new("", 0x51A0CF));
    m.insert("makefile", Icon::new("", 0x6D8086));
    m.insert("markdown", Icon::new("", 0x519ABA));
    m.insert("material", Icon::new("󰔉", 0xB83998));
    m.insert("md", Icon::new("", 0xFFFFFF));
    m.insert("mdx", Icon::new("", 0x519ABA));
    m.insert("mint", Icon::new("󰌪", 0x87C095));
    m.insert("mjs", Icon::new("", 0xF1E05A));
    m.insert("mk", Icon::new("", 0x6D8086));
    m.insert("ml", Icon::new("λ", 0xE37933));
    m.insert("mli", Icon::new("λ", 0xE37933));
    m.insert("mo", Icon::new("∞", 0x9772FB));
    m.insert("mustache", Icon::new("", 0xE37933));
    m.insert("nim", Icon::new("", 0xF3D400));
    m.insert("nix", Icon::new("", 0x7EBAE4));
    m.insert("opus", Icon::new("󰈣", 0xF88A02));
    m.insert("org", Icon::new("", 0x77AA99));
    m.insert("otf", Icon::new("", 0xECECEC));
    m.insert("pck", Icon::new("", 0x6D8086));
    m.insert("pdf", Icon::new("", 0xB30B00));
    m.insert("php", Icon::new("", 0xA074C4));
    m.insert("pl", Icon::new("", 0x519ABA));
    m.insert("pm", Icon::new("", 0x519ABA));
    m.insert("png", Icon::new("", 0xA074C4));
    m.insert("pp", Icon::new("", 0xFFA61A));
    m.insert("ppt", Icon::new("󰈧", 0xCB4A32));
    m.insert("prisma", Icon::new("󰔶", 0xFFFFFF));
    m.insert("pro", Icon::new("", 0xE4B854));
    m.insert("ps1", Icon::new("󰨊", 0x4273CA));
    m.insert("psb", Icon::new("", 0x519ABA));
    m.insert("psd", Icon::new("", 0x519ABA));
    m.insert("psd1", Icon::new("󰨊", 0x6975C4));
    m.insert("psm1", Icon::new("󰨊", 0x6975C4));
    m.insert("py", Icon::new("", 0xFFBC03));
    m.insert("pyc", Icon::new("", 0xFFE291));
    m.insert("pyd", Icon::new("", 0xFFE291));
    m.insert("pyo", Icon::new("", 0xFFE291));
    m.insert("query", Icon::new("", 0x90A850));
    m.insert("r", Icon::new("󰟔", 0x358A5B));
    m.insert("rake", Icon::new("", 0x701516));
    m.insert("rb", Icon::new("", 0x701516));
    m.insert("res", Icon::new("", 0xCC3E44));
    m.insert("resi", Icon::new("", 0xF55385));
    m.insert("rlib", Icon::new("", 0xDEA584));
    m.insert("rmd", Icon::new("", 0x519ABA));
    m.insert("rproj", Icon::new("󰗆", 0x358A5B));
    m.insert("rs", Icon::new("", 0xDEA584));
    m.insert("rss", Icon::new("", 0xFB9D3B));
    m.insert("sass", Icon::new("", 0xF55385));
    m.insert("sbt", Icon::new("", 0xCC3E44));
    m.insert("scala", Icon::new("", 0xCC3E44));
    m.insert("scm", Icon::new("󰘧", 0x000000));
    m.insert("scss", Icon::new("", 0xF55385));
    m.insert("sh", Icon::new("", 0x4D5A5E));
    m.insert("sig", Icon::new("λ", 0xE37933));
    m.insert("slim", Icon::new("", 0xE34C26));
    m.insert("sln", Icon::new("", 0x854CC7));
    m.insert("sml", Icon::new("λ", 0xE37933));
    m.insert("sol", Icon::new("󰞻", 0x519ABA));
    m.insert("spec.js", Icon::new("", 0xCBCB41));
    m.insert("spec.jsx", Icon::new("", 0x20C2E3));
    m.insert("spec.ts", Icon::new("", 0x519ABA));
    m.insert("spec.tsx", Icon::new("", 0x1354BF));
    m.insert("sql", Icon::new("", 0xDAD8D8));
    m.insert("sqlite", Icon::new("", 0xDAD8D8));
    m.insert("sqlite3", Icon::new("", 0xDAD8D8));
    m.insert("styl", Icon::new("", 0x8DC149));
    m.insert("sublime", Icon::new("", 0xE37933));
    m.insert("suo", Icon::new("", 0x854CC7));
    m.insert("sv", Icon::new("󰍛", 0x019833));
    m.insert("svelte", Icon::new("", 0xFF3E00));
    m.insert("svg", Icon::new("󰜡", 0xFFB13B));
    m.insert("svh", Icon::new("󰍛", 0x019833));
    m.insert("swift", Icon::new("", 0xE37933));
    m.insert("t", Icon::new("", 0x519ABA));
    m.insert("tbc", Icon::new("󰛓", 0x1E5CB3));
    m.insert("tcl", Icon::new("󰛓", 0x1E5CB3));
    m.insert("terminal", Icon::new("", 0x31B53E));
    m.insert("test.js", Icon::new("", 0xCBCB41));
    m.insert("test.jsx", Icon::new("", 0x20C2E3));
    m.insert("test.ts", Icon::new("", 0x519ABA));
    m.insert("test.tsx", Icon::new("", 0x1354BF));
    m.insert("tex", Icon::new("󰙩", 0x3D6117));
    m.insert("tf", Icon::new("", 0x5F43E9));
    m.insert("tfvars", Icon::new("", 0x5F43E9));
    m.insert("toml", Icon::new("", 0x6D8086));
    m.insert("tres", Icon::new("", 0xCBCB41));
    m.insert("ts", Icon::new("", 0x519ABA));
    m.insert("tscn", Icon::new("󰎁", 0xA074C4));
    m.insert("tsx", Icon::new("", 0x1354BF));
    m.insert("twig", Icon::new("", 0x8DC149));
    m.insert("txt", Icon::new("󰈙", 0x89E051));
    m.insert("v", Icon::new("󰍛", 0x019833));
    m.insert("vala", Icon::new("", 0x7239B3));
    m.insert("vh", Icon::new("󰍛", 0x019833));
    m.insert("vhd", Icon::new("󰍛", 0x019833));
    m.insert("vhdl", Icon::new("󰍛", 0x019833));
    m.insert("vim", Icon::new("", 0x019833));
    m.insert("vsh", Icon::new("", 0x5D87BF));
    m.insert("vue", Icon::new("", 0x8DC149));
    m.insert("wasm", Icon::new("", 0x5C4CDB));
    m.insert("webmanifest", Icon::new("", 0xF1E05A));
    m.insert("webp", Icon::new("", 0xA074C4));
    m.insert("webpack", Icon::new("󰜫", 0x519ABA));
    m.insert("xcplayground", Icon::new("", 0xE37933));
    m.insert("xls", Icon::new("󰈛", 0x207245));
    m.insert("xlsx", Icon::new("󰈛", 0x207245));
    m.insert("xml", Icon::new("󰗀", 0xE37933));
    m.insert("xul", Icon::new("", 0xE37933));
    m.insert("yaml", Icon::new("", 0x6D8086));
    m.insert("yml", Icon::new("", 0x6D8086));
    m.insert("zig", Icon::new("", 0xF69A1B));
    m.insert("zsh", Icon::new("", 0x89E051));
    m
});
