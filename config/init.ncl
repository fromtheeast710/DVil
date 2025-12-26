let { Editor, Theme, .. } = import "editor.ncl"
in
{
  theme | Theme,
  num_pad = {
    left = 50.0,
    right = 15.0
  },
  color_column.enable = true,
} | Editor
