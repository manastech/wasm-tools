(component
  (type (;0;)
    (instance
      (type (;0;) (func))
      (export (;0;) "foo" (func (type 0)))
    )
  )
  (import "foo" (instance (;0;) (type 0)))
  (type (;1;)
    (instance
      (type (;0;) (func (param "code" u32)))
      (export (;0;) "proc-exit" (func (type 0)))
    )
  )
  (import (interface "foo:foo/my-wasi") (instance (;1;) (type 1)))
  (core module (;0;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param i32 i32) (result i32)))
    (import "foo" "foo" (func (;0;) (type 0)))
    (import "wasi-snapshot-preview1" "proc_exit" (func (;1;) (type 1)))
    (import "wasi-snapshot-preview1" "random_get" (func (;2;) (type 2)))
    (memory (;0;) 1)
    (export "memory" (memory 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module (;1;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i32) (result i32)))
    (import "foo:foo/my-wasi" "proc-exit" (func $proc_exit (;0;) (type 0)))
    (func (;1;) (type 0) (param i32)
      local.get 0
      call $proc_exit
    )
    (func (;2;) (type 1) (param i32 i32) (result i32)
      i32.const 0
    )
    (export "proc_exit" (func 1))
    (export "random_get" (func 2))
  )
  (core module (;2;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i32) (result i32)))
    (func $adapt-wasi-snapshot-preview1-proc_exit (;0;) (type 0) (param i32)
      local.get 0
      i32.const 0
      call_indirect (type 0)
    )
    (func $adapt-wasi-snapshot-preview1-random_get (;1;) (type 1) (param i32 i32) (result i32)
      local.get 0
      local.get 1
      i32.const 1
      call_indirect (type 1)
    )
    (table (;0;) 2 2 funcref)
    (export "0" (func $adapt-wasi-snapshot-preview1-proc_exit))
    (export "1" (func $adapt-wasi-snapshot-preview1-random_get))
    (export "$imports" (table 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module (;3;)
    (type (;0;) (func (param i32)))
    (type (;1;) (func (param i32 i32) (result i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 1)))
    (import "" "$imports" (table (;0;) 2 2 funcref))
    (elem (;0;) (i32.const 0) func 0 1)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance (;0;) (instantiate 2))
  (alias export 0 "foo" (func (;0;)))
  (core func (;0;) (canon lower (func 0)))
  (core instance (;1;)
    (export "foo" (func 0))
  )
  (alias core export 0 "0" (core func (;1;)))
  (alias core export 0 "1" (core func (;2;)))
  (core instance (;2;)
    (export "proc_exit" (func 1))
    (export "random_get" (func 2))
  )
  (core instance (;3;) (instantiate 0
      (with "foo" (instance 1))
      (with "wasi-snapshot-preview1" (instance 2))
    )
  )
  (alias core export 3 "memory" (core memory (;0;)))
  (alias export 1 "proc-exit" (func (;1;)))
  (core func (;3;) (canon lower (func 1)))
  (core instance (;4;)
    (export "proc-exit" (func 3))
  )
  (core instance (;5;) (instantiate 1
      (with "foo:foo/my-wasi" (instance 4))
    )
  )
  (alias core export 0 "$imports" (core table (;0;)))
  (alias core export 5 "proc_exit" (core func (;4;)))
  (alias core export 5 "random_get" (core func (;5;)))
  (core instance (;6;)
    (export "$imports" (table 0))
    (export "0" (func 4))
    (export "1" (func 5))
  )
  (core instance (;7;) (instantiate 3
      (with "" (instance 6))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)