# which-langs

This is a reimplementation of [linquist](https://github.com/github-linguist/linguist).

It helps you identify the programming language of a given source code file.

> Righ now this is a toy project and the main goal here is to use a real project on my nvim setup

## Disambiguation

I am not using the `linguist` name, since it is taken `nuget.org` on by a [localization library]().

## Alternatives

- [linquist](https://github.com/github-linguist/linguist) - the real deal
- [hyperpolyglot](https://github.com/monkslc/hyperpolyglot) - RIIR

## Getting Started

You can install DotLinguist globally as a .NET tool:

```bash
dotnet tool install -g which-langs
```

An run it on a repository folder

```bash
$ which-langs [PATH]
85.00% Rust
15.00% RenderScript
```

