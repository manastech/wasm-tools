(component
  (core module (;0;)
    (type (;0;) (func))
    (func (;0;) (type 0))
    (export "name#a" (func 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core instance (;0;) (instantiate 0))
  (component (;0;)
    (type (;0;) (record))
    (export (;1;) "foo" (type 0))
  )
  (instance (;0;) (instantiate 0))
  (export (;1;) (interface "foo:foo/name") (instance 0))
  (alias export 1 "foo" (type (;0;)))
  (type (;1;) (func (param "f" 0)))
  (alias core export 0 "name#a" (core func (;0;)))
  (func (;0;) (type 1) (canon lift (core func 0)))
  (alias export 1 "foo" (type (;2;)))
  (component (;1;)
    (type (;0;) (record))
    (import "import-type-foo" (type (;1;) (eq 0)))
    (import "import-type-foo0" (type (;2;) (eq 1)))
    (type (;3;) (func (param "f" 2)))
    (import "import-func-a" (func (;0;) (type 3)))
    (export (;4;) "foo" (type 1))
    (type (;5;) (func (param "f" 4)))
    (export (;1;) "a" (func 0) (func (type 5)))
  )
  (instance (;2;) (instantiate 1
      (with "import-func-a" (func 0))
      (with "import-type-foo" (type 2))
      (with "import-type-foo0" (type 0))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
  (export (;3;) "name" (instance 2))
)