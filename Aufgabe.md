# Aufgabe

Aktuell: 
- [x] Edges e statt g(n,m) im Read Me -> Konstistenz
- [ ] Bei p = 1 ist bei option 2 kein i -> j mit i > j, bzw. auch bei anderen p Bevorzugung \
-> Reihenfolge shuffeln bei dem for loop beide vektoren in der geschachtelten schleife
\
-> Laufzeit zweitrangig
- [ ] Random 10-100 computation time
- [ ] Zusatzoption output in extra file für alle task optionen text datei
- [x] Read me -> graphviz installieren
- [ ] Legende subscript letters darstellungsfehler
- [ ] Computation time mit angeben 
- [ ] Uniprocessor -> parallele ausführung der tasks, mit priorität, liste mit schon ausgeführten knoten
- [ ] Ohne datei dot datei -> directory os error beim generieren der svg datei

Zusätzlich:
- [ ] TGFF Version 3.6 anschauen
- [ ] Recherche: Was gibt es neben Erdos Renyji?\
 --> UUnifast Algotithmus anschauen, Paper (Grundlagen für Parameter Knoten Generierung -> erfüllbar) + evt. Abhängigkeiten einfügen
- [ ] (Im Tgff Projekt: Für jeden Knoten Deadline hinzufügen, wenn möglich?)

\
Abgeschlossen:
- [x] Implementieren von [Erdős–Rényi Model](https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model) Graphstruktur Generierung  
- [x] Zyklenfreiheit prüfen
- [x] Parameter (Deadline, Startzeit, Verfügbarkeitszeit = 0, ...) für jeden Task im Graph hinzufügen
- [x] Statt ungerichteten gerichtete Kanten einfügen
- [x] Selber überlegt: Files besser aufteilen -> Lesbarkeit verbessern & kommentieren, Automatische Docs 

\
\
Insgesamt:

- Task Graphstruktur als Directed Acyclic Graph DAG

- Dateiformat mit den Tasks und Abhängigkeiten als svg, dot, textuell, ... 
  => Recherche

- Frontend oder Graphische Oberfläche/ CLI in C++ oder Rust



Nützlicher Link: [Task Graphs For Free TGFF](https://robertdick.org/projects/tgff/) => veralteter build mit gcc 4.3

\
\
Nachgefragt:
- Keine (automatisierten) Tests notwendig, eine Build Pipeline auch nicht
- Code und Kommentare auf Englisch, ReadMe auf deutsch ist okay so
