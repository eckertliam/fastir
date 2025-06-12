# Road Map

Fastir is a research project meant to explore a number of machine learning-driven optimizations for LLVM IR.

## Current State

Currently, Fastir is in the early stages of development. I am building out tooling with [llvm_tools](../llvm_tools) helping me extract features efficiently using Rust. I initially plan to iterate off the work done in [InlineML](https://github.com/eckertliam/inline-ml) and create a tool that can identify inlining opportunities better than LLVM's inlining pass. I cover my plans for this in [Fastir's Inlining Pass](#fastirs-inlining-pass). I cover tooling in [Tooling](#tooling).

- Tooling with `llvm_tools` in Rust + PyO3
- Initial feature extraction in working state (module, function, and basic block level features)

## Tooling

### LLVM Tools

I am using [llvm_tools](../llvm_tools) to help me extract features using Rust. I use a fork of [llvm-ir](https://github.com/cdisselkoen/llvm-ir) my fork is [llvm-ir-fork](https://github.com/eckertliam/llvm-ir). This fork of llvm-ir gives me a Rust-based API to wrangle LLVM without depending heavily on LLVM's C API directly. `llvm_tools` exposes a pythonic interface for extracting features from LLVM IR thanks to PyO3.

### Compiler Gym

In the future I plan to use [Compiler Gym](https://github.com/facebookresearch/CompilerGym) for benchmarking and possibly training. I will definitely be using it for benchmarking, comparing LLVM Opt passes to my own passes and generating nice graphs. I might use it for training as well, but I am not sure yet.

## Fastir's Inlining Pass

I plan to iterate off the work done in [InlineML](https://github.com/eckertliam/inline-ml) and create a tool that can identify inlining opportunities better than LLVM's inlining pass.

### Milestones for Inlining Pass

- [ ] Complete feature extraction for inlining pass returning an Inlining Feature Vector as polars dataframe from `llvm_tools`.
- [ ] Create a model that can identify inlining opportunities.
- [ ] Modify LLVM IR to inline functions based on the model's predictions.
- [ ] Benchmark tooling for comparison to LLVM's inlining pass.

## Future Plans

While inlining is the first target, I plan to explore:
- Loop unrolling
- Vectorization opportunities
- Transformer models for mirroring LLVM's full opt -O3 pass
