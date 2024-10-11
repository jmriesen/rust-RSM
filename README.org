* Project Motivation 
I think that in order to truly understand an idea or a practice you must actually try it, and live with it for a while.

A while back I realized that I was watching a lot of conference talks and reading a lot of blog posts about various software development practices.
This lead to me developing some opinions did not hold up well when theory crashed into reality.

This project is intended to be a sand box where I can try out various ideas and practices, to figure out what works and doesn't.
* Mission Statement 
To learn and explore how best to develop software.

To me best currently means creating software that is correct (has no bugs), I am confident is correct (I can confidently say it has no bugs), and is easy to change. 
* Project Goals
  - Learn how to efficiently refactor code
  - Lean how to efficiently test my code, and test my tests
      - Meaningful unit tests
      - Mutation testing 
      - TODO code coverage
      - A/B testing
      - CICD pipeline
  - Live with the code

     Some ideas seem really convenient at the start, but become a tangled mess later on.
     Others seem overly verbose initially but pay of in the long run.
     One of the reasons I selected RSM is because I new it would be a large project, and I would have to live with any decisions I made.
* What is this Project?
  This project is a port of the [[https://gitlab.com/Reference-Standard-M/rsm][Reference Standard M]] implementation maintained by David Wicksell from C to Rust.
** Why RSM? 
   I have always found language design interesting. 
   When I learned M at work I thought it seemed like a "simple" language and wondered if I could write an interpreter for it.
   I quickly realized that M was not as simple as I assumed, especially when I started to try and add indirection and goto support to my half baked interpreter.
   Eventually I ended up poking around online looking at how other interpreters worked and found RSM.
   On a whim I started trying to convert some of the code to Rust, and after a while the project became the main code base I would use to try stuff out.

* Project Structure
** [[./ffi/][ffi]] 
   The purpose of this create is to store/manage the ordinal C code from RSM.

   This crate is responsible for: 
   - Building the C code
   - Generating the ffi struct and function definitions
   - Exposing an Unsafe API to the C code.
   - Exposing a Safe API to the C code
*** Safe API Considerations
   Most of the other crates use the unsafe API currently, but I would like to move towards only exposing a safe API.
**** Concurrency Considerations
     The original RSM code was a single threaded/multi process application. 
     Because of this there are many references to global variables without any sort of synchronization control.
     Rust by default runs all of its unit test in parallel (multi-threaded) so care has to be taken whenever the Rust unit tests call out to C in order to avoid race conditions.

     Additionally C uses a shared memory segment to manage cross process communication.

**** Type Considerations
     There are of course all the normal C vs Rust type considerations.
     However in addition to that the C code makes heavy use of un-sized types and quasi self references types.

****** Dynamically Sized Types
     One fairly common dynamically sized type used in the C code is the CSTRING.
     The strut definition claims to hold a [u_char;65535] however in practice that is only the max size for this type.
     If the string is smaller then 65535 bytes (most of the time) then the C code only allocates enough space to hold the string.

     Rust does have a way of creating dynamically sized types, but I have not explored that yet.

****** Quisi Self References 
       Self references in Rust are hard since unless you use the Pin type, Rust assumes that everything can be moved.
       Fortunately the C types are only kind of self referential.
       They frequently assume that type A will be immediately followed by type B.
       So we end up with the logical composite type AB, which is impotent for understanding the C code.
       Fortunately this is mostly just a logical construction, and is fairly easy to spot in the C code, and deal with in the Rust rewrite.

** [[./tree-sitter-M][tree-sitter-m]]
   For this project I have chosen to use a [[https://tree-sitter.github.io/tree-sitter/][tree-sitter]] parser.
   - Uses JavaScript to specify the grammar.
   - Uses an [[https://tree-sitter.github.io/tree-sitter/creating-parsers.html#external-scanners][external scanner]] to deal with indentation.
   - Runs the tree-sitter-cli during from the build.rs
   - Generates
     - A C library that contains the parser.
     - A Rust crate that wraps that C library.
     - A [[./tree-sitter-M/src/node-types.json][node-types.json]] file that describes the Grammars structure.
** [[./lang-model/][language model]]
   This crate stores the Rust types wrappers for each of the nodes in the M grammar.
   The [[./lang-model/src/models.rs]] file is generated from the [[./tree-sitter-M/src/node-types.json][node-types.json]] using a separate personal project.
** [[./compiler/][compiler]]
   This crate takes the abstract syntax tree (AST) provided by the lang-model and converts it into byte code.

   This is some regards is the "root" crate. Before I started splitting up the project into multiple crates everything lived in here.
   As a result there are still a number lingering artifacts, type defs/impls that really don't belong in this crate, but still live there since I have not gotten around to cleaning them up.

   Additionally I did not yet understand how the shared memory segment worked when writing most of this crate.
   So for the most part it just pretends that the globals in the shared memory segment don't exist.
** [[./interpreter/][interpreter]]
   This create is going to store the actual interpreter binary.
   Currently the crate is responsible for 
   - Creating a database file
   - Setting up the shared memory segment

   In the future I will probably end up splitting out the shared memory segment out into its own crate since the compiler will eventually need access to it. 

** [[./lang-server][Language Server]]
   This is a language server for M.
   This was a spur of the moment weekend project, and more or less only gives you some basic syntax highlighting/error detection.
   I think there are a lot of neat things you could do with a language server, but I will need a better grip on how the interpreter as a whole works before I can do any of them safely.

   Future feature idea
   - Find all assumed variables and indirection calls.
   - Renaming variables
   - Find all references
   - Lint for unused and assumed variables
   - Extract method 
   - Introduce package scoping

   One of the biggest roadblocks as I see to refactoring in M is the dynamic scoping of variables.
   Dynamic scoping makes it vary difficult to locally reason about renaming variables.
* Running the Project
  This project dose not currently produce a working executable.
  If you need a working M interpreter please see [[https://gitlab.com/Reference-Standard-M/rsm][Reference-Standard-M]].
  Any bugs that I find during the course of creating this clone will be reported back up stream to RSM.
** Development Environment Setup
   NOTE check the [[./.github/workflows/rust.yml][github actions]] for the version of the cli tools
   - cargo install tree-sitter-cli --version <version>  --locked
   - cargo install cargo-mutants  --version <version> --locked
   - You will need clang installed (requirement of bindgen) see bindgens [[https://rust-lang.github.io/rust-bindgen/requirements.html][documentation]] for more details.

** Running Unit Tests 
   - cargo test
** Running Fuzz Tests
   NOTE: currently fuzzing is only done in the symbol table create. 
   - cargo fuzz list
   - cargo fuzz run <fuzzing target>
   [[https://rust-fuzz.github.io/book/cargo-fuzz.html][cargo fuzz book]]

** Running Mutation Testing
   NOTE this can take a while.
   - cd <crate name>
   - cargo mutants

* Techniques/Concepts 
** Unit Testing
    The more unit tests I write the more useful I realize unit tests are, and the less they seem to be about double checking my work.

*** Concept Overview 
    Unit tests are code fragments that describes how a "unit" of code is invoked and what behavior is expected from that "unit".

    I think unit test should be.
    - Descriptive.
      Well written unit test should be able to serve as documentation.
    - Small.
      If you need more then 20 lines of code to write a unit test you are probably violating the single responsibility heuristic. 
    - Simple.
      It should take less the 2 minutes for someone to look at a unit test, understand what it is verifying and why that is correct.
    - Fast and deterministic. 
      Unit test should be run frequently.
      At least once every half an hour, often much more frequently.  

    When I was first introduced to unit testing in collage, it was primarily presented as a afterthought, a way to verity your code was correct before turning in the assignment.
    However waiting to write/run unit test until after the code is already in a finished state robes unit tests of most of there utility.

    As I see it there are two main benefits to writing unit test before writing your code.
   - First if it allows you to imagine how your code will be called. 
      If the unit tests are hard to write then the application code is going to be hard to write/maintain.
   - Second once code behavior has been pined down with unit tests, you can fearlessly refactor without worrying about breaking changes.
      Frequency it is only after a first draft solution that I truly understand the problem I am trying to solve.
      Therefore I will nearly always want to refactor my code at some point in the future.
      With a robust set of unit tests this is a fairly painless simple process.
      Without them I have to be hyper aware of every change I make, as any change could introduce a bug.
** Mutation Testing

*** Concept Overview

   Mutation testing is a technique to check how well a test suite defines the behavior of code base.
   This is accomplished by introducing mutations.
   If the mutated code can still past the test suit, then the tests are not fully specifying the systems behavior.[fn:: It is possible for a mutation to not change systems behavior, but in this project that should be fairly rare.]
   The main downside to mutation testing is that it takes time to run. 
   For each mutation we may have to run the entire test suite. 

*** Use in Rust-RSM
    I am currently using [[https://mutants.rs/][cargo-mutants]] to run mutation testing. 
    Since mutation testing can take quite a while to run, the CICD pipeline is currently only introducing mutations into the edited hunks of code.
    The expectation is that all generated mutants must be killed or timeout before a branch can be merged into main.

*** General thoughts on the Mutation Testing
    Mutation testing is a low effort technique that dramatically increases my confidence in my unit test suite.
    The first time I ran it I ended up finding a bug in my test code that would have been impossible to detect using traditional unit testing.
** C Foreign Function Interface 
***  Concept Overview
    A foreign function is simply a function that was written in a different programming language.
    In this case I am calling C code from Rust and vs versa.
    Calling code that was written in a different language requires some extra care:
    - Parameters must match the target languages memory layout
    - The C/Rust compilers don't understand the other language and therefore have to assume the foreign code could do anything

*** Use in Rust-RSM
    In this project Rust is responsible for matching the C ABI when cross language calls occurs.
    The bindgen and cbindgen tools do most of the heavy lifting by automating the generation of type and function definitions.
    However there are a few project specific things that have to be kept in mind:
    - Don't blindly trust the generated type definitions.
      The C code uses dynamically sized types, however the header files/generated Rust types assume these types occupy their max size.
      CSTRING is the most commonly used example.
    - Pay extra attention to pointers/pointer arithmetic.
      The C code sometimes allocates memory for multiple structs of different types at once.
      This is particularly prominent in the shared memory segment.
      This can be problematic since Rust assumes every struct can be moved, however as long as you are aware of this issue it is fairly easy to work around.
    - The C code assumes it is single threaded.
      The C code uses a lot of global variables, and since it assumes it is single threaded there are no synchronization guards in place (Atomics, Mutexs, ext).
      However Rust unit tests are multi threaded by default.

*** General Thoughts on FFI
    Some times you just need functionality that was written in a another programming language.
    There are a lot of invariants that need to be upheld, but it is manageable with the code gen build tools.
    It is not something I would introduce into a project on a whim, but I would also not be afraid of adding it if I needed some specialized functionality.
** A/B and Fuzz Testing
   A/B and Fuzz Testing are two separate concepts, however in this project I frequently use them together.
*** Concept Overview 
    The idea behind Fuzz testing is that we want to verify some invariant is upheld for all inputs. (Normally that invariant is: The program did not crash and no invalid memory accesses occurred.)
    So we plug in a bunch of random inputs and verify that the invariant holds true. 

    A/B testing at its core is testing the invariant, "system A should behave the same as system B".
*** Use in Rust-RSM
    A lot of my unit tests so far have been A/B tests. 
    Since this is a rewrite it is fairly easy to create A/B tests however I would like to move away from using this as a primary means of testing.
    The fact that I have the original C code that I can A/B test against is fairly artificial, so I think I will learn more by focusing on other forms of unit tests.

    That being said I think A/B testing can be put to great use checking how well I converted/tested a module of code.
    If bugs are slipping past my unit tests and are only being caught once I add A/B tests, this is an indication that my unit test writing skills need additional work.

