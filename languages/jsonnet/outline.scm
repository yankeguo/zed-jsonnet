(local_bind
  (local) @context
  (bind
    .
    (id) @name
    (params)? @context.extra)) @item

(field
  (fieldname
    (id) @name)
  (params)? @context.extra) @item

(field
  (fieldname
    (string
      (string_content) @name))
  (params)? @context.extra) @item
