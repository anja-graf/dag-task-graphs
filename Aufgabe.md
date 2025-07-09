# Aufgabe

Aktuell:  
- [ ] [Modul drs python](https://pypi.org/project/drs/) einbinden, min max mitgeben oder default wert
- [ ] Evt. mit Text file für minimum maximum sequence (max=[], min=[]) => wenn len nicht = knotenanzahl abbruch
- [x] Warnung/Error ausgeben falls nicht möglich bei uunifast
- [ ] [Vortrag](#presentation) vorbereiten 
- [ ] falls generiert mit uunifast: legende anpassen

Zusätzlich:
- TGFF Version 3.6 anschauen
- [x] Recherche: Was gibt es neben Erdos Renyji?\
 --> UUnifast Algotithmus anschauen, Paper (Grundlagen für Parameter Knoten Generierung -> erfüllbar) + evt. Abhängigkeiten einfügen
- (Im Tgff Projekt: Für jeden Knoten Deadline hinzufügen, wenn möglich?)

\
Abgeschlossen:
- [x] Implementieren von [Erdős–Rényi Model](https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model) Graphstruktur Generierung  
- [x] Zyklenfreiheit prüfen
- [x] Parameter (Deadline, Startzeit, Verfügbarkeitszeit = 0, ...) für jeden Task im Graph hinzufügen
- [x] Statt ungerichteten gerichtete Kanten einfügen
- [x] Selber überlegt: Files besser aufteilen -> Lesbarkeit verbessern & kommentieren, Automatische Docs 
- [x] Edges e statt g(n,m) im Read Me -> Konstistenz
- [x] Bei p = 1 ist bei option 1 kein i -> j mit i > j, bzw. auch bei anderen p Bevorzugung \
-> Reihenfolge shuffeln bei dem for loop beide vektoren in der geschachtelten schleife
\
-> Laufzeit zweitrangig
- [x] Random 10-100 computation time
- [x] Zusatzoption output in extra file für alle task optionen text datei
- [x] Read me -> graphviz installieren
- [x] Legende subscript letters darstellungsfehler
- [x] Computation time mit angeben 
- [x] Uniprocessor -> parallele ausführung der tasks, mit priorität, liste mit schon ausgeführten knoten
- [x] Ohne datei dot datei -> directory os error beim generieren der svg datei

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

\
\
<span id="presentation">Vortrag</span>:
- Präsentation mit Folien => Vorlagen verwenden
- Umfang: 15 Minuten reden + evt. Fragen beantworten
- Erstelle Graphen zB in Folie packen (muss keine live demo sein)
- Inhalt: "Kurz vorstellen, was das Ziel des FMs war, welche Verfahren du dir dafür
angeschaut hast, und dann eben die Ergebnisse etwas präsentieren"
