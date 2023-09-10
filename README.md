# F1 TSP
F1-tsp is a project to calculate the fastest distance around all of the f1 locations. To do this it currently uses the nearest neighbor algorithm which is admitedly not very optimized, I am looking into implementing better algorithms.

## WHY❓
I did this to learn rust and graphviz along with a small bit of trigonometry (have a look at distance.rs). It serves no actual purpose due to the F1 calendar not being based on shortest distances between tracks but if the FIA want to use this optimized calendar, feel free!

## RUN🏃
#### WITH CARGO🚚:
```bash
git clone https://github.com/max-amb/f1-tsp.git && cd f1-tsp
cargo run # To build and run 
```
#### WITHOUT CARGO⛔🚚:
- go to releases and download the latest binary
```bash
chmod +x {YOUR BINARY}
./{YOUR BINARY}
```
## BUILD👷:
```bash
git clone https://github.com/max-amb/f1-tsp.git && cd f1-tsp
cargo build # To build the binary
```

## COMING UP⏭️
- [] Basic TUI user interface
- [] Held-Karp algorithm implementation
- [] A branch and bound algorithm implementation

## ACKNOWLEDGEMENTS🙏
- Thank you to bacinger for the json file provided at his [f1-circuits](https://github.com/bacinger/f1-circuits) repository, I use a modified version within this program!
