- [Background](#background)
- [Analysis of Rust as a programming language](#analysis-of-rust-as-a-programming-language)
  - [Classification of the language family](#classification-of-the-language-family)
  - [Readability, write-ability, orthogonality, cost, and reliability](#readability-write-ability-orthogonality-cost-and-reliability)
  - [Implementation method](#implementation-method)
  - [Static and dynamic aspects](#static-and-dynamic-aspects)
  - [Best suited projects](#best-suited-projects)
  - [Data types and control structures](#data-types-and-control-structures)
  - [Support for polymorphism](#support-for-polymorphism)
  - [Support for object-oriented programming and inheritance](#support-for-object-oriented-programming-and-inheritance)
  - [Use of pointers/references](#use-of-pointersreferences)
  - [Memory management](#memory-management)
  - [Functions/sub-programs implementation](#functionssub-programs-implementation)
  - [Support for lambda functions and capturing variables](#support-for-lambda-functions-and-capturing-variables)
  - [Stand-out features](#stand-out-features)
- [Creating a grep-like tool](#creating-a-grep-like-tool)
  - [Project Setup](#project-setup)
- [Takeaways](#takeaways)


## Background
The purpose of this project is to explore the Rust programming language in order to analyze it's strengths and weaknesses. Going into this project, I had zero experience with Rust, so this project will double as a learning experience.

## Analysis of Rust as a programming language
### Classification of the language family
Rust is a multi-paradigm language that supports imperative procedural, concurrent, and functional styles.

### Readability, write-ability, orthogonality, cost, and reliability

* Readability: Rust’s syntax is designed to be clear and concise. It uses meaningful naming conventions and well-structured code.
* Write-ability: Rust has a concise syntax, easy-to-remember keywords and constructs, and powerful abstractions that make it easy to express complex ideas.
* Orthogonality: Rust’s features are designed to work well together, reducing the likelihood of programmer errors.
* Cost: The cost of using Rust can be higher due to its steep learning curve. However, its focus on safety and performance can lead to lower costs in the long run.
* Reliability: Rust’s strong static typing and ownership model ensure memory safety and thread safety, eliminating many classes of bugs at compile time.
* Syntax and semantics rules: Rust’s syntax is similar to C and C++, but it enforces strict borrowing and lifetime rules to ensure memory safety. It also supports pattern matching with the match keyword.

### Implementation method
Rust uses ahead-of-time (AOT) compilation. This means Rust code is compiled to machine code before it is run, resulting in efficient execution.

### Static and dynamic aspects
Rust is statically typed and uses a sophisticated system of lifetimes to manage memory. It also supports dynamic dispatch through traits.

### Best suited projects
Rust is well-suited for system programming, game development, and other performance-critical applications. It’s also used in web development and for creating command-line tools.

### Data types and control structures
Rust has several built-in data types, including integers, floating-point numbers, Booleans, characters, tuples, arrays, and slices. Control structures include if, else, while, for, and match.

### Support for polymorphism
Rust supports polymorphism through traits and generics. Traits define shared behavior, and generics allow for code that works over many types.

### Support for object-oriented programming and inheritance
While Rust has features similar to OOP, such as methods and encapsulation, it does not support inheritance.

### Use of pointers/references
Rust uses references and raw pointers for direct memory access. It also has unique pointers for heap allocation.

### Memory management
Rust uses a system of ownership with a set of rules that the compiler checks at compile time. It does not have a garbage collector.

### Functions/sub-programs implementation
Functions are declared with the fn keyword. Methods are associated functions defined within an impl block.

### Support for lambda functions and capturing variables
Rust supports lambda functions, known as closures. Closures can capture variables from their surrounding scope.

### Stand-out features
Rust’s most notable feature is its focus on safety without sacrificing performance. Its rich type system and ownership model guarantee memory safety and thread safety.

## Creating a grep-like tool
To get hands-on experience with Rust's features, I have created a command line tool modeling Linux's grep.

### Project Setup
After installing Rust on my system and an extension for VSCode, I was ready to create the project. Rust comes with a package manager called cargo. Running the cargo new command in the terminal created the initial project files and initialized git source control for the directory. Cargo uses a simple Cargo.toml manifest file for package metadata for compilation.

## Takeaways