# Notizen

Mit diesem Programm lassen sich verschiedene Graphen generieren. Dafür kann mittels `cargo run [ARGS]` im `erdosrenyimodel` Ordner das Projekt kompiliert und ausgeführt werden. 
Alternativ kann auch mit `cargo build --release` ein Binary erstellt werden und dann mit
`./target/release/erdosrenyimodel [ARGS]` aufgerufen werden.

Es stehen verschiedene Optionen für [ARGS] zur Verfügung:

```
The erdosrenyimodel tool generates a random task graph using one of two variants of the Erdős–Rényi model:
    G(n, p) – a graph with n nodes where each edge is included with independent probability p.
    G(n, m) – a graph with n nodes and m edges chosen uniformly at random.
For more information on the Erdős–Rényi model, see also https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model

Usage: erdosrenyimodel [OPTIONS] --nodes <NODES>

Options:
  -n, --nodes <NODES>              Number of nodes
  -m, --edges <EDGES>              Number of edges [default: 2]
  -p, --probability <PROBABILITY>  Probability for edges [default: 0.5]
  -d, --dot <DOT>                  Path for output file in dot format [default: graph.dot]
  -s, --svg <SVG>                  Path for output file in svg format [default: graph.svg]
  -o, --option <OPTION>            Variant selector 1 for G(n,p) or 2 for G(n,m) [default: 1]
  -h, --help                       Print help
```

Zuletzt kann man eine Html Dokumentation für das Projekt mit `cargo doc --open` generieren und unter dem Link öffnen.