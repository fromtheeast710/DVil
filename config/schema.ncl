{
  Nat = std.contract.from_predicate (fun n => std.number.is_integer n && n >= 0),

  U8 = std.contract.from_predicate (fun n => std.number.is_integer n && n >= 0 && n <= 255),

  by_text_width : Number -> Number = fun n => n * 9.85,

  rgb8
    | U8 -> U8 -> U8 -> RGB8
    = fun r g b => { include [r, g, b] },

  rgba8
    | U8 -> U8 -> U8 -> U8 -> RGBA8
    = fun r g b a => { include [r, g, b, a] },

  Hex =
    std.contract.from_predicate (fun str =>
      std.is_string str
      && std.string.is_match "^#([0-9a-f]{3,4}){1,2}$" str
    ),

  Channel =
    std.contract.from_predicate (fun value =>
      std.is_number value
      && std.number.is_integer value
      && value >= 0
      && value <= 255
    ),

  RGB8 = {
    r | Channel,
    g | Channel,
    b | Channel,
  },

  RGBA8 = {
    r | Channel,
    g | Channel,
    b | Channel,
    a | Channel,
  },

  Color =
    std.contract.any_of [
      Hex,
      RGB8,
      RGBA8
    ],
}
