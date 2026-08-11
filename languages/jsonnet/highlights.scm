; Comments
(comment) @comment

; Identifiers (more specific patterns below override this one)
(id) @variable

; Literals
(null) @constant.builtin
; Note: escape sequences cannot be highlighted separately, the grammar's
; scanner folds them into string_content.
(string) @string
(number) @number
[
  (true)
  (false)
] @boolean

; Keywords
"for" @keyword.repeat
"in" @keyword.operator
"function" @keyword.function
[
  "if"
  "then"
  "else"
] @keyword.conditional
[
  (local)
  (tailstrict)
  "assert"
  "error"
] @keyword

; Imports
[
  (import)
  (importstr)
] @keyword.import

; Builtins
[
  (dollar)
  (self)
  (super)
] @variable.special
((id) @variable.special
 (#eq? @variable.special "std"))

; Operators
[
  (multiplicative)
  (additive)
  (bitshift)
  (comparison)
  (equality)
  (bitand)
  (bitxor)
  (bitor)
  (and)
  (or)
  (unaryop)
] @operator

; Punctuation
[
  "["
  "]"
  "{"
  "}"
  "("
  ")"
] @punctuation.bracket

[
  "."
  ","
  ";"
  ":"
] @punctuation.delimiter

[
  "::"
  ":::"
] @punctuation.special

; The "+" of a "field+:" merge expression
(field
  (fieldname) "+" @punctuation.special)

; Field names are properties, not variables
(fieldname (id) @property)
(fieldname
  (string
    (string_start) @emphasis.strong
    (string_content) @property
    (string_end) @emphasis.strong))

; Function and method definitions
(bind function: (id) @function)
(field
  function: (fieldname (id) @function))
(field
  function: (fieldname
    (string
      (string_start) @emphasis.strong
      (string_content) @function
      (string_end) @emphasis.strong)))

; Parameters, both in definitions and in named call arguments
(param
  identifier: (id) @variable.parameter)
(named_argument
  (id) @variable.parameter)

; Function calls
(functioncall
  .
  (id) @function.call)
(functioncall
  (fieldaccess
    last: (id) @function.call))
(functioncall
  (fieldaccess_super
    (id) @function.call))

; Emphasize implicit plus usage (adjacent objects merged without "+")
(implicit_plus
  (_ "}"? @emphasis.strong)
  (object
    "{" @emphasis.strong))
