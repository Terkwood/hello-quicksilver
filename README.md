# Quicksilver Clock Demo

This is a demo of [quicksilver](https://www.ryanisaacg.com/quicksilver/).
It does nothing more than run a slightly modified example of the stopwatch
code.  This version has been changed to show the current time using
[stdweb](https://github.com/koute/stdweb), to use different colors, and
to run in a smaller resolution.

## Live demo

[You can run the demo in your browser right now.](https://clock.prawn.farm)

## Building static assets 

You must first install `wasm-opt`.  [See the instructions in the binaryen package for help](https://github.com/WebAssembly/binaryen#building).  `wasm-opt` needs to be visible in your path.

Then:

```sh
sh build.sh
```

Note that this will create a gzipped version of the compiled webassembly. If you want to serve it locally, you'll need to present the file with the following headers, so that your browser can decode it. 

```
Content-Encoding: gzip
Content-Type: application/wasm
```
