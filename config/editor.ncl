let { rgb8, Nat, Color, .. } = import "schema.ncl" in
{
  # TODO: support 24 colors
  Theme = {
    bg | Color,
    fg | Color,
    text | Color,
  },

  Editor = {
    theme
      | Theme
      | default
      | doc m%"set editor theme
           default: one-dark"%
      = {
        bg = "#aaa",
        fg = "#aaaaaa",
        text = rgb8 0 0 0,
      },

    font_size | Number | default = 13,

    # scrolloff

    num_pad
      | default
      | doc "set the right number padding"
      = {
        left | Number | default = 0.0,
        right | Number | default = 0.0,
      },

    # char_dim
    #   | default
    #   | doc "set monospace font character"
    #   = {
    #     width | Number | default = 0.0,
    #     height | Number | default = 0.0,
    #   },

    color_column
      | optional
      | doc "set the color column"
      = {
        enable | Bool | default = false,
        color | Color | default = theme.fg,
        pos | Nat | default = 80,
      },
  },

  Env = {
    config_dir = [""],
    plugin_dir = ["", ""],
  }
}
