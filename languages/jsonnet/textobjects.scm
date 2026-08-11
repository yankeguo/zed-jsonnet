; Adjacent comments count as one text object
(comment)+ @comment.around

; Functions: local function bindings
(bind
  function: (id)
  (params)
  body: (_) @function.inside) @function.around

; Functions: object methods
(field
  function: (fieldname)
  (params)
  (_) @function.inside) @function.around

; Anonymous functions
(anonymous_function
  body: (_) @function.inside) @function.around

; Objects are the largest structural unit in Jsonnet
(object
  "{"
  (_)* @class.inside
  "}") @class.around
