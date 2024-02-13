-- source: https://github.com/neovim/neovim/blob/master/runtime/lua/vim/shared.lua#L333
local function can_merge(v)
  return type(v) == 'table' and (vim.tbl_isempty(v) or not vim.tbl_isarray(v))
end
local function tbl_extend(behavior, deep_extend, ...)
  if behavior ~= 'error' and behavior ~= 'keep' and behavior ~= 'force' then
    error('invalid "behavior": ' .. tostring(behavior))
  end

  -- if select('#', ...) < 2 then
  --   error(
  --     'wrong number of arguments (given '
  --       .. tostring(1 + select('#', ...))
  --       .. ', expected at least 3)'
  --   )
  -- end

  local ret = {}
  if vim._empty_dict_mt ~= nil and getmetatable(select(1, ...)) == vim._empty_dict_mt then
    ret = vim.empty_dict()
  end

  for i = 1, select('#', ...) do
    local tbl = select(i, ...)
    vim.validate({ ['after the second argument'] = { tbl, 't' } })
    if tbl then
      for k, v in pairs(tbl) do
        if deep_extend and can_merge(v) and can_merge(ret[k]) then
          ret[k] = tbl_extend(behavior, true, ret[k], v)
        elseif behavior ~= 'force' and ret[k] ~= nil then
          if behavior == 'error' then
            error('key found in more than one map: ' .. k)
          end -- Else behavior is "keep".
        else
          ret[k] = v
        end
      end
    end
  end
  return ret
end

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
    tbl_extend = tbl_extend,
    validate = function() end,
    tbl_isempty = function(t)
      assert(type(t) == 'table', string.format('Expected table, got %s', type(t)))
      return next(t) == nil
    end,
    tbl_isarray = function(t)
      if type(t) ~= 'table' then
        return false
      end

      local count = 0

      for k, _ in pairs(t) do
        -- Check if the number k is an integer
        if type(k) == 'number' and k == math.floor(k) then
          count = count + 1
        else
          return false
        end
      end

      if count > 0 then
        return true
      else
        -- TODO(bfredl): in the future, we will always be inside nvim
        -- then this check can be deleted.
        if vim._empty_dict_mt == nil then
          return false
        end
        return getmetatable(t) ~= vim._empty_dict_mt
      end
    end
}
_G.vim = vim

-- load file from stdin and extract icons
local web_devicons = require('nvim-web-devicons')

local default_icon = web_devicons.get_default_icon()
print("pub(crate) static DEFAULT_ICON: Lazy<Icon> = Lazy::new(|| Icon::new(\"" .. default_icon.icon .. "\", 0x" .. default_icon.color:upper():sub(2) .. "));")
print("")

local maps = { 'icons_by_filename', 'icons_by_file_extension' }
for _, map_name in ipairs(maps) do
    local icons = web_devicons[map_name]
    assert(icons ~= nil, "icons not found for " .. map_name)
    local upper = map_name:upper()

    print("pub(crate) static " .. upper .. ": Lazy<HashMap<&str, Icon>> = Lazy::new(|| {")

    local inserts = {}
    for k, v in pairs(icons) do
        inserts[#inserts + 1] = "    m.insert(\"" .. k .. "\", Icon::new(\"" .. v.icon .. "\", 0x" .. v.color:upper():sub(2) .. "));"
    end
    table.sort(inserts)
    print("    let mut m = HashMap::with_capacity(" .. #inserts .. ");")
    print(table.concat(inserts, "\n"))

    print("  m")
    print("});")
    print("")
end
