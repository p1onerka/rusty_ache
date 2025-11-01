# Rusty Ache
Blazingly fast game engine written in Rust.

## How to run
Clone this repo:
```bash
  git clone git@github.com:p1onerka/rusty_ache.git
```
Open the project:
```bash
  cd rusty_ache
```
Run the project:
```bash
  cargo run --bin main
```

## Demo

[Demo](resources/demo.mov)

## Docs
Project documentation is availible [here](https://p1onerka.github.io/rusty_ache/rusty_ache/index.html).

## Performance

Below is a graph showing the dependence of FPS on the number of simultaneously rendered objects, created under the following conditions:

- Setup: MacBook M1, 8Gb RAM
- Data: 113x113px object 
- Resolution: 200x200px

![Performance diagram](resources/perf_diag.png)

As shown on the graph, the engine’s performance is above 20 FPS with <=40 objects. It is strongly discouraged to add more than this number of objects with a size similar to 113x113px on a range smaller than resolution parameters.

## Devs
- [Aleksei Dmitrievstev](https://github.com/admitrievtsev)
- [Ksenia Kotelnikova](https://github.com/p1onerka)
- [Sofya Kozyreva](https://github.com/sofyak0zyreva)
- [Kostya Oreshin](https://github.com/sevenbunu)