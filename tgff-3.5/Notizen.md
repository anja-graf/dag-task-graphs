# Notizen

Das tgff-3.5 wurde so angepasst, dass es nun kompatibel ist mit dem aktuellen GCC C++ Compiler **15.1.1**.

Um es zu kompilieren führe `make` in dem tgff-3.5 Ordner aus.

Dannach gibt es verschiedene Optionen (siehe README):

- Führe `tgff` aus um Informationen zum Befehl zu erhalten

- Durch Argument wie z.B. `tgff simple` werden mit vorliegender `tgffopt` die entprechenden festgelegten Optionen ausgeführt 
  
  => siehe examples Ordner, dort gibt es unter anderem auch das `RunMe` Skript, womit direkt zehn Beispiele generiert werden

- Um das tgff Format in eine .dot Datei umzuwandeln gibt es das Perl Skript `tgff2dot.pl` und kann danach zB. mittels graphviz in ein Png Format umgewandelt werden: 
  `./tgff2dot.pl simple.tgff` 
  
  `dot -Tpng simple.tgff.dot -o simple.png` 
