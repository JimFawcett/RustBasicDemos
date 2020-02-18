# RustBasicDemos
  https://JimFawcett.github.io/RustBasicDemos  - repository documentation

Simple code demos: code structure, types, file_io, ...
I am starting to look seriously at Rust, and will be writing lots of demo code to be sure that I understand. 

For now, the demos are:
<ol>
  <li>
    <strong>data_types</strong> includes simple probes for each of the Rust data types.
  </li>
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
    <strong>display library</strong> provides functions to display the type and value of a variable passed as an argument.
    Libraries can&apos;t write to the console, so library tests are deferred to a separate binary.
  </li>
  <li>
    <strong>display_test</strong> uses the local display crate to test writing information to the console.
  </li>
  <li>
    <strong>file_io</strong> reads from a file and displays results.  will add file writes soon.
  </li>
</ol>
