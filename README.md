# file-web-devicons

Simple Rust binary that reads lines from `stdin`, and prepends icons as defined
in the excellent
[nvim-web-devicons](https://github.com/nvim-tree/nvim-web-devicons).

Handles color codes as well, using the icon colors from the aforementioned
library. Intended to take in output from [fd](https://github.com/sharkdp/fd), to
then be used as input for [fzf-lua](https://github.com/ibhagwan/fzf-lua) in
Neovim.

I found that, in large Git projects, `fzf-lua`'s default `files` provider would
have a delay of several seconds before files would show up, however running `fd`
or `fzf` directly would feel much snappier. I concluded (correctly) that this is
because of the extra processing `fzf-lua` is doing to, amongst other things,
prepend the file icon. By using a custom `fzf-lua` action that invokes `fd` and
pipes the results through this binary, you still get the file icons, but with
the snapiness of running `fd`/`fzf` directly.

You could use this project in Neovim using `fzf-lua` as follows:

```lua
local function fzf_files()
  fzf.fzf_exec('fd --type f --strip-cwd-prefix | /path/to/file-web-devicon', {
    actions = fzf.defaults.actions.files,
    previewer = 'builtin',
  })
end

vim.keymap.set('n', '<C-p>', fzf_files)
```

_Note_: This project has been tested only on MacOS. I don't know whether it will
work on Windows, Linux, or e.g. with non-UTF8 path inputs.
