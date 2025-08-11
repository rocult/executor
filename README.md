# Rocult Executor

A Roblox executor base written engirely in Rust, aiming to be an initial proof of concept. Open sourced to be as a learning resource, as a way to give back to the resources I used to learn. The idea is that if the executor is Rust-based, it'll have all
the advantages Rust does over C++.

## ⚠️ NOTICE ⚠️

I do not work on this anymore, I might pick it up in the future, but it **does not** work in its current state.

## Goals

- [ ] Basic L8 execution
- [ ] Execution on actors
- [ ] 100% Stability
- [ ] TP Handler
- [ ] Basic custom functions
- [ ] Output redirection

## Non-goals

- Injection
    > This is a base, you must bring your injector
- Undetected
    > Open sourcing something undetected is begging it to be patched
- 100% sUNC
    > To avoid more copied executors, there will be almost no custom functions. However, the framework for them
will be there.

## Credits

- https://github.com/0Zayn/ExecutorBase/tree/master
