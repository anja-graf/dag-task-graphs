# Short Description of this project: 
A rust command-line tool for generating random directed acyclic task graphs.
As a starting point Erdős–Rényi models G(n,p) and G(n,m) are used in combination with cycle detection mechanism. From there a priority-based execution order is determined and all task parameters are assigned, including priority, arrival time, start time, computation time, finishing time, absolute deadline, relative deadline, response time, lateness. Lastly, the final scheduled task graph is output and visualized using Graphviz. Additionally, processor utilization can optionally be assigned using UUniFast and DRS algorithms.



# Generierung von Task Graphen 

Mit diesem Programm lassen sich verschiedene Task Graphen generieren. Dafür kann mittels `cargo run [ARGS]` im `task_graphs` Ordner das Projekt kompiliert und ausgeführt werden. 
Alternativ kann auch mit `cargo build --release` ein Binary erstellt werden und dann mit
`./target/release/task_graphs [ARGS]` aufgerufen werden.

Zum Generieren der svg Outputdatei wird [graphviz](https://graphviz.org/) verwendet und sollte daher zuvor installiert werden (getestet mit graphviz Version 12.0.0 bis Version 13.1.2). 

Falls drs benutzt wird, sollte außerdem das [drs python Modul](https://pypi.org/project/drs/) installiert und aufrufbar sein.

Es stehen verschiedene Optionen für [ARGS] zur Verfügung:

```
The task_graphs tool generates random task graphs using one of two variants of the Erdős–Rényi model:
    G(n, p) – a graph with n nodes where each edge is included with independent probability p.
    G(n, m) – a graph with n nodes and m edges chosen uniformly at random.
For more information on the Erdős–Rényi model, see also https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model

Usage: task_graphs [OPTIONS] --nodes <NODES>

Options:
  -n, --nodes <NODES>                Number of nodes
  -m, --edges <EDGES>                Number of edges [default: 2]
  -p, --probability <PROBABILITY>    Probability for edges [default: 0.5]
  -d, --dot <DOT>                    Path for output file in dot format [default: graph]
  -s, --svg <SVG>                    Path for output file in svg format [default: graph]
  -P, --png <PNG>                    Path for output file in png format [default: graph]
  -c, --csv <CSV>                    Path for parameter output file in csv format [default: Not generated]
  -g, --graphs <GRAPHS>              Number of graphs [default: 1]
  -t, --uunifast <UTILIZATION>       Total processor utilization for uunifast [default: Not used]
  -T, --drs <UTILIZATION>            Total processor utilization for drs [default: Not used]
  -U, --upper-bounds <U1,U2,...,UN>  Sequence with upper bounds for each node [default: Not used]
  -L, --lower-bounds <L1,L2,...,LN>  Sequence with lower bounds for each node [default: Not used]
  -D, --drs-path <DRS_PATH>          Path for drs input file [default: Not used]
  -o, --option <OPTION>              Variant selector 1 for G(n,p) or 2 for G(n,m) [default: 1]
  -h, --help                         Print help
```

Zuletzt kann man eine Html Dokumentation für das Projekt mit `cargo doc --open` generieren.
