# Ranlux rs
![](https://img.shields.io/badge/language-Rust-orange)
[![](https://img.shields.io/badge/doc-Read_Me-blue)](https://abouttefeux.github.io/ranlux_rs/ranlux_rs/index.html)
![Build](https://img.shields.io/github/workflow/status/ABouttefeux/ranlux_rs/Rust)
![](https://img.shields.io/github/license/ABouttefeux/ranlux_rs)

Implementation of [Ranlux](https://luscher.web.cern.ch/luscher/ranlux/) in rust.

Ranlux is a non cryptographic pseudo random number generator. It has excellent statistical property.
It was developed for Monte Carlo simulation, more precisely lattice QCD.

# Usage

Add `ranlux_rs = { version = "0.1.0", git = "https://github.com/ABouttefeux/ranlux_rs", branch = "develop" }` in your `cargo.toml`.

# Why ?
I need this generator for [lattice_qcd_rs](https://github.com/ABouttefeux/lattice-qcd-rs) and there was no implementation that I know of in rust.
