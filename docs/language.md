# Language Reference

This page describes RR syntax and semantics based on `src/syntax/*`, `src/hir/*`, and MIR lowering rules.

## Keywords

- `fn`, `let`
- `if`, `else`
- `while`, `for`, `in`
- `return`, `break`, `next`
- `match`
- `import`, `export`
- literals: `true/false`, `null`, `na` (case-insensitive forms supported for booleans/null/na in lexer)

## Literals

- Integer: `1`, `42`, `1L`
- Float: `1.0`, `.5`
- String: `"text"` (with escaped forms)
- Boolean: `true`, `false`
- Null: `null`
- NA: `na`
- Vector literal: `[1, 2, 3]`
- Record literal: `{a: 1, b: 2}`

## Statements

- Variable declaration: `let x = expr;`
- Assignment: `x = expr;` and `x <- expr;`
- Function declaration: `fn f(a, b) { ... }`
- Control flow: `if`, `while`, `for`
- Return: `return expr;` or `return;`
- Loop control: `break;`, `next;`
- Module: `import "path.rr";`, `export fn ...`

## Expressions

- Unary: `-x`, `!x`
- Binary: `+ - * / % %*% == != < <= > >= && ||`
- Range: `a .. b`
- Call: `f(x, y)`, named args `f(x = 1, y = 2)`
- Index: `x[i]`, `m[i, j]`
- Field: `rec.a`
- Lambda: `fn(x) { return x + 1; }`
- Pipe: `x |> f(1)`
- Try postfix: `expr?`
- Match: `match (v) { ... }`
- Column/unquote syntax tokens: `@name`, `^expr` (lowered through HIR tidy/unquote forms)

## Operator Notes

- `%*%` is recognized as matrix multiplication token.
- `&&` and `||` both map to logical operators.
- `|>` is parsed and lowered as call rewriting.

## Pattern Matching

Supported pattern kinds:

- wildcard `_`
- literal patterns
- variable binding
- list pattern `[a, b, ..rest]`
- record pattern `{a: x, b: 1}`

Current limitation:

- record rest pattern (`{a: x, ..rest}`) is not supported.

## Semicolon Policy

Semicolons are optional across statement boundaries, except when two statements are on the same line.

If a new statement starts on the same line without `;`, parser raises:

- `Missing ';' before ... on the same line`

## Assignment Policy (`let` vs `<-`)

RR accepts both `=` and `<-` assignment operators.

If assigning to an undeclared variable:

- default: implicit declaration is allowed and warning is collected
- strict mode (`RR_STRICT_LET=1` or `RR_STRICT_ASSIGN=1`): treated as compile error

## Functions and Closures

- Global functions are lowered from `fn name(...) { ... }`.
- Lambda expressions are lambda-lifted by HIR lowering.
- Captures are packed via runtime helpers:
  - `rr_closure_make`
  - `rr_call_closure`

## Dynamic Builtins and Hybrid Handling

Calls to dynamic runtime features are marked as `unsupported_dynamic` in MIR and handled conservatively:

- `eval`, `parse`, `get`, `assign`, `exists`, `mget`, `rm`, `ls`
- `parent.frame`, `environment`, `sys.frame`, `sys.call`, `do.call`

These functions still emit runnable R code, but optimization is intentionally restricted for safety.
