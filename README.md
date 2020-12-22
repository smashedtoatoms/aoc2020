# aoc2020

## My attempts at Advent of Code 2020 in Rust

I originally started this endeavor in Rust.  I completed days 1, 2, 3, 6, and 8
in rust before getting distracted and falling so far behind that I decided to do
some of it in common lisp as well. I will likely not finish the exercises. It
did force me to enhance my common lisp tooling a bit, and those enhancements
made it into my [sbcl installer](https://github.com/smashedtoatoms/asdf-sbcl).
After blowing time on that the existential crisis began, I did a pros and cons
list of Rust vs Lisp, and decided to really dig in on Rust.  Everything with
Common Lisp feels like I am swimming against the current.  I'll write about it
at some point and add a link here.

## Requirements to build and run everything

- Use [rustup](https://rustup.rs/) to install rust and cargo.  That's it.

## How to Build/Test

- To test: `cargo test`
- To build: `cargo build --release`
- To clean: `cargo clean`
- Seriously, that's it.

## How to Dev

I use vscode with
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
and [Better
TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml).
Start up vscode and get to work.  Again, that's it.

## What did I learn about Rust?

- Rust is not as hard of a language as I had been lead to believe.  The rust
  language is not as hard as the Common Lisp abomination that is it's "build
  system."  If you think lifetimes and the borrow checker are hard, try to build
  a fucking binary for hello world in Common Lisp.  I am partially kidding, but
  holy shit the tooling for common lisp fucking blows.  It's not hard, there is
  just no obvious correct way to do anything.
- Tests co-located with code make me nearly as productive as having a REPL.
- Cargo is a HUGE win.  I have never had such incredible dev ergonomics.
- Workspaces took 10 minutes to get my head around.  They allow my directory
  structure to be whatever it needs to be for a given project.  Imagine that,
  node people, you bunch of fucking nihilists.

## What did I learn about Common Lisp?

Common Lisp Packages/Systems/Modules are an affront to god and a blight on
  humanity.  Seriously, WHAT THE FUCK?!?!  The language itself is fun, and I
  love the repl, but neither are worth the trouble.  At least when I beat the
  borrow checker, I get stable/fast/efficient code.  After I write my own
  installer for SBCL that includes what should be the official package manager,
  I still have to write or update 3 libraries that have been abandoned for 3+
  years.  Ugh.  I want to write lisp so bad, but I don't want to become the sole
  maintainer for 12 abandoned "libraries" to do it.
- Having the compiler and runtime be the same thing is fucking cool.  You start
  your lisp, and you are in the compiler and the runtime.  Because of that, the
  ergonomics are different.  You don't need the cli as much because you can do
  all of it from within the running lisp instance.  You start the
  compiler/runtime, and you give it things to compile as you go.  If you have
  evaluated an expression in your runtime, your compiler/runtime has access to
  it.  This means you can read a file off the disk into a variable in your repl,
  and use that variable to experiment with until you quit.  It's sweet, and it's
  why it is so fun to experiment in lisp.  It's also why no one writes tests and
  abandons projects once it does exactly what the author needs with no regard
  for completeness or correctness.
- "Packages" are not what you are thinking.  In Common Lisp, they are simply
  containers for
  [symbols](https://www.cs.cmu.edu/Groups/AI/html/cltl/clm/node27.html). They're
  purely a mechanism for namespacing symbols.  You can't "load" them.  Get that
  silly notion out of your head.
- "Systems" are code along with some instructions on how said code should be
  built.  A system will say what other systems it depends on, and what should be
  loaded in what order.  You can kind of think of systems like a Makefile
  written in lisp.  SBCL comes with a system definition tool called
  [ASDF](https://common-lisp.net/project/asdf/) which abstracts a bunch of
  commonly used operations in the defining of systems; however it does nothing
  to help you actually get your environment into a state where it can do its
  job.  The concept of a "system" is as old as I am, so it assumes that you've
  already downloaded all of your dependencies and put them in the right place...
  typically all the same place, system-wide, with no regard for reproducible
  builds or project isolation.  Quicklisp tries to solve it by letting you
  download monthly released libraries that are shared across your whole system.
  So not only can't you pin them, but you can only update them for every project
  on your computer once a month.  I have no idea how this is reasonable.  To
  avoid that mess, I use [clpm](https://www.clpm.dev/) and
  [lake](https://github.com/takagi/lake) with my
  [asdf-vm](https://asdf-vm.com/#/) (not to be confused with
  [ASDF](https://common-lisp.net/project/asdf/))
  [plugin](https://github.com/smashedtoatoms/asdf-sbcl).  Lake is basically
  [make](https://www.gnu.org/software/make/manual/make.html) with common lisp
  syntax and runtime.  It's cool, but requires roswell to build unless you want
  to dissect it (which I did, and added to my installer so no one else has to).
  Roswell is trying to be for Common Lisp what cargo is to rust, but it closes
  over it in an awkward way.  It's close, but something is still off.  Clpm is a
  proper package manager like people in 2020 are used to. Why it and not
  insert-thing-you-prefer? Because its Project Goals, which can be read
  [here](https://www.clpm.dev/#project-goals), are obvious, complete,
  modern, and exactly what I want/expect.  Nothing else I have seen lets me have
  reproducible builds per project by default without depending on tools that
  close over things in ways I don't like.  Nothing in either of those projects
  close over things or force workflows upon their users or break existing
  workflows.
- Modules are deprecated.  I have spent no time thinking about them and it
  hasn't affected me yet.  I pray that continues.
- Libraries aren't really a thing.  You will find some "libraries" on
  [quicklisp](https://www.quicklisp.org/beta/),
  [sourceforge](https://sourceforge.net/), [github](https://github.com/), and
  [UltraLisp](https://ultralisp.org) that have ASDF definitions and sanely
  defined packages that will behave how you would expect if you are coming from
  any other programming language on earth. There is nothing standard that
  enforces it though. Some people have just done the right thing and you have
  benefitted from it. Try to bring that kind of kindness into the world when you
  write code because your discipline and kindness are the only things enforcing
  the mental construct of a "library" that you just experienced.  Your evil
  doppelganger is in a time machine on their way to 16 years ago, where they
  will hand-roll a "system" with package names that don't match the system in
  any way, using module definitions that are incompatible with the version of
  lisp you need, and publishing it to source forge with no documentation.
- The community can't decide where it wants to go.  Roswell is closing over the
  entire lisp world in a non-composable way.  I know cargo does it for rust too,
  but cargo does it so well that it's beautiful.  Maybe roswell will get there.
  Quicklisp and Ultralisp are handling distribution and download, but not really
  dealing with repeatable builds, project isolation, version pinning etc.  The
  async world is fucked. The most recent async http client hasn't been updated
  in over 2 years.  The aws sdk hasn't been updated on over a year, and uses a
  non async http library. Nothing is standardized and all of the docs assume
  you've been writing common lisp for your entire life. I used vscode with Alive
  because emacs blows, whether anyone wants to admit it or not (I say this as
  someone who has a very bespoke emacs config that I both love and hate).  I
  want to build some serverless stuff because I don't have time to write an
  async web framework that works well enough to hook a graphql websocket
  interface to (that I will also have to write).  It's an enormous lift to get
  Common Lisp to a sane place, so fuck it, onward with rust.

I seriously tried.  I don't have the energy to dump into this chaos hole.  Maybe
one day I will be wealthy enough to start a company to fix all of this and just
force my opinion on the planet by delivering documented best-practice tooling
for common lisp.  Until then I am off to battle the borrow checker.
