# Ranlux rs
![](https://img.shields.io/badge/language-Rust-orange)
[![](https://img.shields.io/badge/doc-Read_Me-blue)](https://abouttefeux.github.io/ranlux_rs/ranlux_rs/index.html)
![Build](https://img.shields.io/github/workflow/status/ABouttefeux/ranlux_rs/Rust)
![](https://img.shields.io/github/license/ABouttefeux/ranlux_rs)

Implementation of [Ranlux](https://luscher.web.cern.ch/luscher/ranlux/) in rust. Moreover improved generator are provided
following the blog post of [Christoph Conrads' blog](https://christoph-conrads.name/faster-ranlux-pseudo-random-number-generators/).



Ranlux is a non cryptographic pseudo random number generator. It has excellent statistical property.
It was developed for Monte Carlo simulation, more precisely Lattice Field Theory Simulations.


The base implementation of ranlux is meant to generator f32 and f64 random number between 0 and 1. We provided a modified a modified version able to generate random bites.

# Usage

Add `ranlux_rs = { version = "0.1.0", git = "https://github.com/ABouttefeux/ranlux_rs", branch = "develop" }` in your `cargo.toml`.

# Why ?
I need this generator for [lattice_qcd_rs](https://github.com/ABouttefeux/lattice-qcd-rs) and there was no implementation that I know of in rust.

# Sources
- [Ranlux](https://luscher.web.cern.ch/luscher/ranlux/)
- [Martin Lüscher, A Portable High-Quality Random Number Generator for Lattice Field Theory Simulations: [arXiv:hep-lat/9309020] ](https://arxiv.org/pdf/hep-lat/9309020.pdf)
- [Christoph Conrads, Faster RANLUX Pseudo-Random Number Generators](https://christoph-conrads.name/faster-ranlux-pseudo-random-number-generators/)
