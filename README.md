# Quicksilver Clock Demo

This is a demo of [quicksilver](https://www.ryanisaacg.com/quicksilver/).
It does nothing more than run a slightly modified example of the stopwatch
code.  This version has been modified to show the current time using
[stdweb](https://github.com/koute/stdweb), and to use different colors.

## Building static assets 

```sh
sh build.sh
```

Note that this will create a gzipped version of the compiled webassembly. If you want to serve it locally, you'll need to present the file with the following headers, so that your browser can decode it. 

```
Content-Encoding: gzip
Content-Type: application/wasm
```
