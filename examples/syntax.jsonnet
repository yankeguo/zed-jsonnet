/*
 * Syntax showcase for the Jsonnet language.
 * Used to verify highlighting, folding, outline and text objects.
 */

// A line comment.
# Another style of line comment.

// Local variables and functions, including multiple bindings.
local pour = 1.5,
      garnish = 'Maraschino Cherry';
local mix(a, b=2) = a * b;
local ids = [x for x in std.range(1, 3) if x % 2 == 1];

local base = {
  kind: 'drink',
  served:: 'Tall',            // hidden field
  recipe+:: ['stir'],         // hidden field, merged with +=
  describe(name):: |||
    A %s, served %s.
  ||| % [name, self.served],
};

{
  // Imports
  local lib = import 'lib.libsonnet',
  local text = importstr 'notes.txt',

  // Fields: identifiers, strings and computed names
  simple: 42,
  'quoted field': "double quoted",
  [std.format('dyn-%d', 1)]: true,

  // Numbers and literals
  int: 1024,
  exp: 1.2e3,
  nothing: null,
  flags: [true, false],

  // Strings: verbatim and text block
  verbatim: @'no \n escape',
  block: |||
    multi-line
    text
  |||,

  // Operators
  arith: mix(3) + 4 - 2 * 1.5 / 0.5 % 2,
  bits: (240 & 15) | 1 ^ 2 << 3 >> 1,
  logic: !false && true || 'a' in { a: 1 },
  cmp: if 1 < 2 && 2 <= 2 && 3 > 2 && 3 >= 3 && 1 != 2 then 'yes' else 'no',
  neg: -pour,

  // Conditionals, assertions and errors
  checked:
    assert pour > 0 : 'pour must be positive';
    if std.type(self.nothing) == 'null' then 'ok' else error 'boom',

  // Object inheritance and references
  manhattan: base {
    kind: 'Manhattan',
    ingredients: [
      { kind: 'Rye', qty: mix(2, 0.5) },
      { kind: 'Sweet Red Vermouth', qty: 1 },
      { kind: 'Angostura', qty: 'dash' },
    ],
    garnish: garnish,
    note: super.kind + ' over ' + self.served,
    root: $.simple,
  },

  // Implicit plus: adjacent objects are merged
  collins: { base: 'gin' } { garnish: 'cherry' },

  // Array and object comprehensions
  squares: [x * x for x in ids],
  inverted: { [f]: f for f in std.objectFields(self.manhattan) },

  // Anonymous functions, named arguments and tailstrict
  total: std.foldl(function(acc, x) acc + x, self.squares, 0),
  named: std.objectHasEx(self.manhattan, 'kind', true),
  strict: std.sum(std.range(1, 10)) tailstrict,

  // Indexing and slicing
  first: self.squares[0],
  slice: self.squares[1:3],
}
