# Project Motivation 
I think that in order to truly understand an idea or a practice you must actually try it, and live with it for a while.

A while back I realized that I was watching a lot of conference talks and reading a lot of blog posts about various software development practices.
This lead to me developing some opinions that did not hold up well when theory crashed into reality.

This project is intended to be a sandbox where I can try out various ideas and practices, to figure out what works and doesn't.
# Mission Statement 
To learn and explore how best to develop software.

To me, best currently means creating software that is correct (has no bugs), I am confident is correct (I can confidently say it has no bugs), and is easy to change. 
# Project Goals
- Learn how to efficiently refactor code
- Lean how to efficiently test my code, and test my tests
- Live with the code

Some ideas seem really convenient at the start, but become a tangled mess later on.
Others seem overly verbose initially but pay off in the long run.
One of the reasons I selected RSM for this porting project is because I knew it would be a large project, and I would have to live with any decisions I made.

# What is this Project?
  This project is a port of the [Reference Standard M](https://gitlab.com/Reference-Standard-M/rsm) implementation maintained by David Wicksell from C to Rust.

## Why RSM?
I have always found language design interesting.
When I learned M at work, I thought it seemed like a "simple" language and wondered if I could write an interpreter for it.
I quickly realized that M was not as simple as I assumed, especially when I started to try and add indirection and goto support to my half-baked interpreter.
Eventually I ended up poking around online looking at how other interpreters worked and found RSM.
On a whim I started trying to convert some of the code to Rust, and after a while the project became the main codebase I would use to try stuff out.

# Project Structure
## [ffi](./ffi/) 
The purpose of this crate is to store/manage the ordinal C code from RSM and the Foreign Function Interface (ie making C and Rust play nicely together in the same codebase).

This crate is responsible for:
- Building the C code
- Generating the ffi struct and function definitions
- Exposing an Unsafe API to the C code
- Exposing a Safe API to the C code

### Safe API Considerations
Most of the other crates use the unsafe API currently, but I would like to move towards only exposing a safe API.

#### Concurrency Considerations
The original RSM code was a single threaded/multi process application.
Because of this, there are many references to global variables without any sort of synchronization control.
Rust by default runs all of its unit tests in parallel (multi-threaded) so care has to be taken whenever the Rust unit tests call out to C in order to avoid race conditions.

Additionally, C uses a shared memory segment to manage cross process communication.

#### Type Considerations
There are of course all the normal C vs Rust type considerations.
However, in addition to that, the C code makes heavy use of un-sized types and quasi self references types.

###### Dynamically Sized Types
One fairly common dynamically sized type used in the C code is `CSTRING`.
The struct definition claims to hold a `[u_char;65535]`, however in practice that is only the max size for this type.
If the string is smaller than 65535 bytes (most of the time) then the C code only allocates enough space to hold the string.

Rust does have a way of creating dynamically sized types, but I have not explored that yet.

###### Quasi Self References
Self references in Rust are hard since unless you use [Pin](https://doc.rust-lang.org/std/pin/index.html), Rust assumes that everything can be moved.
Fortunately, the C types are only kind of self referential.
They frequently assume that type A will be immediately followed by type B.
So we end up with the logical composite type AB.
This is mostly just a logical construction, and is fairly easy to spot in the C code and handle once you know what to look for. 

## [tree-sitter-m](./tree-sitter-M)
For this project, I have chosen to use a [tree-sitter](https://tree-sitter.github.io/tree-sitter/) parser.
This crate is responsible for:
- Using JavaScript to specify the grammar

- Using an [external scanner](https://tree-sitter.github.io/tree-sitter/creating-parsers.html#external-scanners) to deal with indentation
- Running the `tree-sitter-cli` as part of the [build script](./tree-sitter-M/bindings/rust/build.rs)
- Generating
  - A C library that contains the parser
  - A Rust crate that wraps that C library
  - A [node-types.json](./tree-sitter-M/src/node-types.json) file that describes the grammar's structure

## [lang-model](./lang-model/)
This holds the Rust types wrappers for each of the nodes in the M grammar.
The [models.rs](./lang-model/src/models.rs) file is generated from the [node-types.json](./tree-sitter-M/src/node-types.json)using a separate personal project.

## [compiler](./compiler/)
This crate takes the abstract syntax tree (AST) provided by the `lang-model` and converts it into byte code.

This is in some regards the "root" crate. Before I started splitting up the project into multiple crates, everything lived in here.
As a result, there are still a number of lingering artifacts, type definitions and implementation blocks, that really don't belong in this crate, but still live there since I have not gotten around to cleaning them up.

Additionally, I did not yet understand how the shared memory segment worked when I started writing this crate, so for the most part, it just pretends that the globals in the shared memory segment don't exist.

## [interpreter](./interpreter/)
Currently, the crate is responsible for:
- Creating a database file
- Setting up the shared memory segment

## [Language Server](./lang-server)
This is a language server for M.
This was a spur-of-the-moment weekend project, and more or less only gives you some basic syntax highlighting/syntax error detection.
I think there are a lot of neat things you could do with a language server.

Future feature ideas:
- Find all assumed variables and indirection calls.
- Renaming variables
- Find all references
- Lint for unused and assumed variables
- Extract method
- Introduce package scoping

One of the biggest roadblocks as I see to refactoring in M is the dynamic scoping of variables.

Example of dynamic scoping:
```
A()
   s i=i+1 ; tag A references to variable i without initializing it
   q
B() 
   s i=0
   d A()
   q ; i now has the value of 1

C() 
   s i=9
   d A()
   q ; i now has the value of 10
```
Dynamic scoping makes it difficult to locally reason about the code.
This makes it rather hard to create automatic refactoring tools that preserve behavior even for relatively simple operations like "rename variable".

# Running the Project
This project does not currently produce a working executable.
If you need a working M interpreter, please see [Reference-Standard-M](https://gitlab.com/Reference-Standard-M/rsm).
Any bugs that I find during the course of creating this clone will be reported back upstream to RSM.
## Development Environment Setup
NOTE check the [GitHub actions](./.github/workflows/rust.yml) for the version of the cli tools
- `cargo install tree-sitter-cli --version <version> --locked`
- `cargo install cargo-mutants   --version <version> --locked`
- You will need clang installed (requirement of bindgen) see bindgen [documentation](https://rust-lang.github.io/rust-bindgen/requirements.html) for more details.

## Running Unit Tests 
- `cargo test`
## Running Fuzz Tests
NOTE: currently fuzzing is only done in the symbol table create. 
- `cargo fuzz list`
- `cargo fuzz run <fuzzing target>`

[cargo fuzz book](https://rust-fuzz.github.io/book/cargo-fuzz.html)

## Running Mutation Testing
NOTE: this can take a while.
- `cd <crate name>`
- `cargo mutants`

# Techniques/Concepts 
## Unit Testing
The more unit tests I write the more useful I realize unit tests are, and the less they seem to be about double checking my work.

### Concept Overview 
Unit tests are code fragments that describes how a "unit" of code is invoked and what behavior is expected from that "unit".

I think unit test should be.
- Descriptive.
Well written unit test should be able to serve as documentation.
- Small.
If you need more than 20 lines of code to write a unit test you are probably violating the single responsibility heuristic. 
- Simple.
It should take less the 2 minutes for someone to look at a unit test, understand what it is verifying and why that is correct.
- Fast and deterministic. 
Unit test should be run frequently.
At least once every half an hour, often much more frequently.  

When I was first introduced to unit testing in collage, it was primarily presented as an afterthought, a way to verify your code was correct before turning in the assignment.
However waiting to write/run unit test until after the code is already in a finished state robs unit tests of most of there utility.

As I see it there are two main benefits to writing unit test before writing your code.
- First if it allows you to imagine how your code will be called. 
If the unit tests are hard to write then the application code is going to be hard to write/maintain.
- Second once code behavior has been pined down with unit tests, you can fearlessly refactor without worrying about breaking changes.
Frequency it is only after a first draft solution that I truly understand the problem I am trying to solve.
Therefore I will nearly always want to refactor my code at some point in the future.
With a robust set of unit tests this is a fairly painless and simple process.
Without them I have to be hyper aware of every change I make, as any change could introduce a bug.
## Mutation Testing

### Concept Overview
Mutation testing is a technique to check how well a test suite defines the behavior of code base.
This is accomplished by introducing mutations.
If the mutated code can still past the test suit, then the tests are not fully specifying the systems behavior.
(It is possible for a mutation to not change systems behavior, but in this project that should be fairly rare.)
The main downside to mutation testing is that it takes time to run. 
For each mutation we may have to run the entire test suite. 

### Use in Rust-RSM
I am currently using [cargo-mutants](https://mutants.rs/) to run mutation testing. 
Since mutation testing can take quite a while to run, the CICD pipeline is currently only introducing mutations into the edited hunks of code.
The expectation is that all generated mutants must be killed or timeout before a branch can be merged into main.

### General thoughts on the Mutation Testing
Mutation testing is a low effort technique that dramatically increases my confidence in my unit test suite.
The first time I ran `cargo mutants`, I ended up finding a bug in the test code that would have been impossible to detect using traditional unit testing.
## C Foreign Function Interface 
###  Concept Overview
A foreign function is simply a function that was written in a different programming language.
In this case I am calling C code from Rust and vice versa.
Calling code that was written in a different language requires some extra care:
- Parameters must match the target language's memory layout
- The C/Rust compilers don't understand the other language and therefore have to assume the foreign code could do anything

### Use in Rust-RSM
In this project Rust is responsible for matching the C ABI when cross language calls occurs.
The bindgen and cbindgen tools do most of the heavy lifting by automating the generation of type and function definitions.
However there are a few project specific things that have to be kept in mind:
- Don't blindly trust the generated type definitions.
The C code uses dynamically sized types, however the header files/generated Rust types assume these types occupy their max size.
`CSTRING` is the most commonly used example.
- Pay extra attention to pointers/pointer arithmetic.
The C code sometimes allocates memory for multiple structs of different types at once.
This is particularly prominent in the shared memory segment.
This can be problematic since Rust assumes every struct can be moved, however as long as you are aware of this issue it is fairly easy to work around.
- The C code assumes it is single threaded.
The C code uses a lot of global variables, and since it assumes it is single threaded there are no synchronization guards in place (Atomics, Mutexs, ext).
However Rust unit tests are multi threaded by default.

### General Thoughts on FFI
Some times you just need functionality that was written in another programming language.
There are a lot of invariants that need to be upheld, but it is manageable with the bindgen and cbindgen build tools.
It is not something I would introduce into a project on a whim, but I would also not be afraid of adding it if I needed some specialized functionality.

## A/B and Property Based Testing
A/B and Property Testing are two separate concepts, however in this project I frequently use them together.
### Concept Overview 
The idea behind Property Based Testing is that we want to verify some invariant is upheld for all inputs. 
So we plug in a bunch of random inputs and verify that the invariant holds true. 

A/B testing at its core is testing the invariant, "system A should behave the same as system B".

Note: Fuzz Testing can be view as Property Based Testing with the invariant: The code does not crash and there are no memory access violations.
### Use in Rust-RSM
A lot of my unit tests so far have been A/B tests. 
Since this is a rewrite it is fairly easy to create A/B tests however I would like to move away from using this as a primary means of testing.
The fact that I have the original C code that I can A/B test against is fairly artificial, so I think I will learn more by focusing on other forms of testing.

That being said I think A/B testing can be put to great use checking how well I converted/tested a module of code.
If bugs are slipping past my unit tests and are only being caught once I add A/B tests, this is an indication that my unit test writing skills need additional work.
