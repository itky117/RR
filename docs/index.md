---
layout: home

hero:
  name: RR
  text: An Optimizing Compiler for R
  tagline: R-first syntax with SSA-based MIR and Tachyon optimizer. Documentation is implementation-driven from compiler source code.
  actions:
    - theme: brand
      text: Getting Started
      link: /getting-started
    - theme: alt
      text: Language Reference
      link: /language

features:
  - title: R-First Syntax
    details: Write with familiar R conventions like <code><-</code>, <code>function()</code>, dotted identifiers, pattern matching, and pipes.
  - title: Tachyon Optimizer
    details: SSA-based MIR with SCCP, GVN, LICM, vectorization, inlining, TCO, and bounds-check elimination across -O0/-O1/-O2 levels.
  - title: Code-Accurate Docs
    details: Language and runtime docs track parser/lowering behavior in <code>src/syntax</code>, <code>src/hir</code>, and <code>src/mir</code>.
---
