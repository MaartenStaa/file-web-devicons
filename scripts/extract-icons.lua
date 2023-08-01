-- shim for called vim functions
local vim = {
    api = {
        nvim_create_autocmd = function() end,
        nvim_set_hl = function() end,
    },
    o = {
        background = "dark"
    },
    F = {
        if_nil = function(x, y) if x == nil then return y else return x end end,
    },
    tbl_extend = function() end,
}
_G.vim = vim

-- load file from stdin and extract icons
local web_devicons = dofile()

local default_icon = web_devicons.get_default_icon()
print("pub static DEFAULT_ICON: Lazy<Icon> = Lazy::new(|| Icon::new(\"" .. default_icon.icon .. "\", \"" .. string.sub(default_icon.color, 2) .. "\"));")
print("")

local maps = { 'icons_by_filename', 'icons_by_file_extension' }
for _, map_name in ipairs(maps) do
    local icons = web_devicons[map_name]
    assert(icons ~= nil, "icons not found for " .. map_name)
    local upper = map_name:upper()

    print("pub static " .. upper .. ": Lazy<HashMap<&str, Icon>> = Lazy::new(|| {")
    print("    let mut m = HashMap::new();")

    local inserts = {}
    for k, v in pairs(icons) do
        inserts[#inserts + 1] = "    m.insert(\"" .. k .. "\", Icon::new(\"" .. v.icon .. "\", \"" .. string.sub(v.color, 2) .. "\"));"
    end
    table.sort(inserts)
    print(table.concat(inserts, "\n"))

    print("  m")
    print("});")
    print("")
end
