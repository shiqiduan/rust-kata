// Crate and module
// 代码的结构和组织管理

// 概念
// Crate ：独立可编译单元，编译后生成一个可执行文件和库。核心为了共享，可以被发布和使用
//      类型：lib 类型、二进制可执行。
// Package（项目）：包含独立的 cargo.toml文件，可以包含一个到多个crate。
//      二进制 package
//          cargo new my-project - Created binary (application) `my-project` package
//      库 package
//          cargo new my-lib --lib - Created library `my-lib` package

// 所以，crate 和 package 的区别是什么呢？怎么定义和 crate 呢？

// Package 结构
// ├── Cargo.toml
// ├── Cargo.lock
// ├── src
// │   ├── main.rs
// │   ├── lib.rs
// │   └── bin
// │       └── main1.rs
// │       └── main2.rs
// ├── tests
// │   └── some_integration_tests.rs
// ├── benches
// │   └── simple_bench.rs
// └── examples
//     └── simple_example.rs

// 多 crate 的结构。默认 package 里面，仅包含一个同名的 crate
// MyProject/
// ├── Cargo.toml
// ├── crate1/
// │   ├── Cargo.toml
// │   └── src/
// │       └── lib.rs
// └── crate2/
// ├── Cargo.toml
// └── src/
// └── lib.rs

// Module
// 路径问题 - self、super、crate
// 1. 绝对路径， use crate::basics::primitive_types::demo;
// 2. 相对路径，
// 代码可见性 - pub
// 模块的定义和声明可以分离
// 如果是目录，需要创建mod.rs or 目录同名.rs
