(component
  (type (;0;)
    (instance
      (type (;0;) (record))
      (export (;1;) "name" (type (eq 0)))
    )
  )
  (import (interface "foo:foo/name1") (instance (;0;) (type 0)))
  (alias export 0 "name" (type (;1;)))
  (type (;2;)
    (instance
      (alias outer 1 1 (type (;0;)))
      (export (;1;) "name" (type (eq 0)))
    )
  )
  (import (interface "foo:foo/name2") (instance (;1;) (type 2)))
  (core module (;0;)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core instance (;0;) (instantiate 0))
  (alias export 0 "name" (type (;3;)))
  (alias export 1 "name" (type (;4;)))
  (component (;0;)
    (type (;0;) (record))
    (import "import-type-name" (type (;1;) (eq 0)))
    (import "import-type-name0" (type (;2;) (eq 1)))
    (export (;3;) "name" (type 1))
    (export (;4;) "name1" (type 2))
  )
  (instance (;2;) (instantiate 0
      (with "import-type-name" (type 3))
      (with "import-type-name0" (type 4))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
  (export (;3;) "name" (instance 2))
)