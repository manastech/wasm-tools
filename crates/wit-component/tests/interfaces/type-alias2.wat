(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (flags "b"))
          (export (;1;) "a" (type (eq 0)))
          (export (;2;) "b" (type (eq 1)))
          (type (;3;) (func (result 2)))
          (export (;0;) "f" (func (type 3)))
        )
      )
      (export (;0;) (interface "foo:foo/foo") (instance (type 0)))
      (type (;1;)
        (component
          (type (;0;)
            (instance
              (type (;0;) (flags "b"))
              (export (;1;) "a" (type (eq 0)))
              (export (;2;) "b" (type (eq 1)))
              (type (;3;) (func (result 2)))
              (export (;0;) "f" (func (type 3)))
            )
          )
          (export (;0;) (interface "foo:foo/foo") (instance (type 0)))
        )
      )
      (export (;0;) (interface "foo:foo/my-world") (component (type 1)))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
  (export (;1;) (interface "foo:foo/wit") (type 0))
)