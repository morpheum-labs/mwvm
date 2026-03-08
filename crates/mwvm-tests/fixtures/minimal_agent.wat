(module
  (memory (export "memory") 1)

  ;; Dummy function that calls our host functions (for parity tests)
  (func (export "morpheum_infer")
    (param i32 i32 i32 i32)
    (result i32)
    i32.const 0   ;; return 0 (success) for test purposes
  )

  (func (export "morpheum_vector_search")
    (param i32 i32 i32 i32 i32)
    (result i32)
    i32.const 0
  )

  (func (export "morpheum_store_context")
    (param i32 i32 i32 i32)
    (result i32)
    i32.const 0
  )
)