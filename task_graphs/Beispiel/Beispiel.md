# Beispielaufruf von taskgraphs mit --nodes 6 --edges 4 --option 2 --graphs 2 --csv graph --drs 1

Es sollen 2 Graphen mit 6 Knoten und 4 Kanten durch die G(n,m) Erdos Renyi Variante generiert werden und zusätzlich zum normalen Output ein Parameter File namens 'graph' erstellt werden.

---

Als erstes werden die 6 Knoten zufällig auf die 2 Graphen verteilt. Das Terminal zeigt:

```
Nodes were distributed on graphs: [2, 4]
```

D.h. Graph 1 bekommt 2 Knoten, Graph 2 bekommt 4 Knoten.

---

Als nächstes werden die 4 Kanten zufällig verteilt, da '--option 2' gewählt wurde. Dabei wird berücksichtigt, dass zB Graph 1 nur zwei Knoten hat und maximal 1 Kante bekommen kann um danach als DAG generiert werden zu können.

Das Terminal zeigt:

```
Edges were distributed on graphs: [1, 3]
```

D.h. Graph 1 bekommt 1 Kante, Graph 2 bekommt 3 Kanten. 

Zusammenfassend:

```
The graph 1 will be generated with option 2, nodes = 2 and edges = 1
The graph 2 will be generated with option 2, nodes = 4 and edges = 3
```

----

Als nächstes wird die Utilization '--drs 1' berechnet:

```
Distributing total processor utilization U = 1 with drs
Generating default upper bounds that ensure the task has an execution time of at most 100
Generating default lower bounds that ensure the task has an execution time of at least 10
Lower bounds: [0.02304147465437788, 0.02304147465437788, 0.02304147465437788, 0.02304147465437788, 0.02304147465437788, 0.02304147465437788] 
Upper bounds: [0.2304147465437788, 0.2304147465437788, 0.2304147465437788, 0.2304147465437788, 0.2304147465437788, 0.2304147465437788]
Computed vector: [0.09140234828266663, 0.030362562166607723, 0.22793928929622265, 0.21722891624566287, 0.20910871308123183, 0.2239581709276084]
```

Da keine bounds mitgegeben wurden, wird 10/T_i als lower bound und 100/T_i als upper bound verwendet. Dabei ist T_i die Periode einer Task i, wobei die Perioden auf die Summe aller Computation times gesetzt werden, d.h. alle Tasks haben dieselbe Periode.

Danach werden die entsprechenden U_i Werte für die Graphen ausgewählt und die Computation times angepasst, damit das Verhältnis stimmt. Außerdem werden die Graphen gespeichert.

```
Subvector for utilization of graph 1: [0.09140234828266663, 0.030362562166607723]
Parameter file successfully created 'graph1.csv'
Graph 1 saved as 'graph1.dot', 'graph1.svg' and 'graph1.png'


Subvector for utilization of graph 2: [0.22793928929622265, 0.21722891624566287, 0.20910871308123183, 0.2239581709276084]
Parameter file successfully created 'graph2.csv'
Graph 2 saved as 'graph2.dot', 'graph2.svg' and 'graph2.png'
```

---

graph1.png und graph1.csv:

<img src="graph1.png" title="" alt="Graph1" data-align="center">

| Task/Priority | Start time | Computation time | Absolute deadline | Relative deadline | Finishing time | Response time | Lateness | Arrival time | Period | Utilization        |
| ------------- | ---------- | ---------------- | ----------------- | ----------------- | -------------- | ------------- | -------- | ------------ | ------ | ------------------ |
| 0             | 0          | 40               | 434               | 434               | 40             | 40            | -394     | 0            | 434    | 0.0921658986175115 |
| 1             | 40         | 13               | 434               | 434               | 53             | 53            | -381     | 0            | 434    | 0.0299539170506912 |

---

graph2.png und graph2.csv:

<img src="graph2.png" title="" alt="Graph2" data-align="center">

| Task/Priority | Start time | Computation time | Absolute deadline | Relative deadline | Finishing time | Response time | Lateness | Arrival time | Period | Utilization       |
| ------------- | ---------- | ---------------- | ----------------- | ----------------- | -------------- | ------------- | -------- | ------------ | ------ | ----------------- |
| 0             | 282        | 99               | 434               | 434               | 381            | 381           | -53      | 0            | 434    | 0.228110599078341 |
| 1             | 0          | 94               | 434               | 434               | 94             | 94            | -340     | 0            | 434    | 0.216589861751152 |
| 2             | 94         | 91               | 434               | 434               | 185            | 185           | -249     | 0            | 434    | 0.209677419354839 |
| 3             | 185        | 97               | 434               | 434               | 282            | 282           | -152     | 0            | 434    | 0.223502304147465 |
