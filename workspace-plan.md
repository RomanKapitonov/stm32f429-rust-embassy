# Workspace Structure Plan

## New Directory Structure:
```
stm32f429-example/
├── Cargo.toml              (workspace root)
├── firmware/               (renamed from src/)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── channel.rs
│   │   ├── ffi.rs
│   │   └── init.rs
│   ├── build.rs
│   └── ...
└── led-effects/            (new library crate)
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── rgb.rs
        └── effects/
            ├── mod.rs
            ├── envelope.rs
            ├── envelopes/
            ├── builder.rs
            ├── generator.rs
            ├── modifier.rs
            └── parameter.rs
```

## Step-by-step Instructions:

### 1. Create workspace root Cargo.toml
### 2. Create firmware/ subdirectory
### 3. Create led-effects/ library crate
### 4. Move code between crates
### 5. Update imports
### 6. Test compilation
