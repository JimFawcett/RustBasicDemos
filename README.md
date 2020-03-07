# RustBasicDemos
  https://JimFawcett.github.io/RustBasicDemos  - repository documentation

Simple code demos: code structure, types, file_io, ...
I am starting to look seriously at Rust, and will be writing lots of demo code to be sure that I understand. 
Now, these demos are just an unmanicured exploration trail.  Once I know more I will trim them to be useful for
others trying to climb the same path.

For now, the demos are:
<ol>
  <li>
    <strong>data_lifecycle</strong - incomplete explorations of ownership and borrowing.
  </li>
  <li>
    <strong>data_types</strong> includes simple probes for each of the Rust data types.
  </li>
  <li>
    <strong>demo_lib and demo_test</strong> - simple demonstration of using sister library and other files in src.
  </li>
  <li>
    <strong>display library</strong> provides functions to display the type and value of a variable passed as an argument.
    Libraries can&apos;t write to the console, so library tests are deferred to a separate binary.
  </li>
  <li>
    <strong>display_test</strong> uses the local display crate to test writing information to the console.
  </li>
  <li>
    <strong>file_io</strong> reads from a file and displays results.  will add file writes soon.
  </li>
  <li>
    <strong>function_probes</strong> - passing arguments including functions
  <li>
    <strong>generic_probes</strong> - generic functions and structs
  <li>
    <strong>ops_probes</strong> - just starting, will explore functions and lambdas.
  <li>
    <strong>rust_modules</strong> - hello world for modules.
  <li>
    <strong>string_probes</strong> - many small experiments that need trimming
  <li>
    <strong>rust_probes</stron> demos for
    <ul>
      <li>
        variations on print! formatting
      </li>
      <li>
        std::Debug formatting for Rust types
      </li>
      <li>
        formatting floats showing use of format flags for exp notation, precision, alignment in field, ...
      </li>
      <li>
        copy, move, clone, Drop trait, Clone trait.  Includes simple but useful module demo.
      </li>
    </ul>
  </li>
  <li>
    <strong>vector_probes</strong> - a few simple demos.
</ol>
