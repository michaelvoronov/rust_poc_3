## rust_poc_3

```bash
cargo build --target wasm32-unknown-unknow --release
```

produces module with only three imported functions:
```bash
(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32) (result i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32 i32) (result i32)))
  (import "sqlite" "store" (func $store (type 0)))
  (import "sqlite" "load" (func $load (type 1)))
  (import "sqlite" "deallocate" (func $deallocate (type 0)))
  (func $__wasm_call_ctors (type 2))
  (func $allocate (type 1) (param i32) (result i32)
    i32.const 0)
  (func $invoke (type 3) (param i32 i32) (result i32)
    i32.const 0)
  (func $foo (type 2)
    i32.const 1
    i32.const 1
    call $store
    i32.const 1
    call $load
    drop
    i32.const 1
    i32.const 1
    call $deallocate)
  (table (;0;) 1 1 anyfunc)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "__heap_base" (global 1))
  (export "__data_end" (global 2))
  (export "allocate" (func $allocate))
  (export "invoke" (func $invoke))
  (export "foo" (func $foo)))
```
