# pir-Abschlussprojekt

Commits: 
  -auf Master nur kompilierende Sachen commiten
  -englische Commits
  -beschreibung nur wenn nötig


Ideen Doc: https://docs.google.com/document/d/1I86jCiSxkUiOldzZ_XSB1ZsVTcj7TMt-E8Wmi-W6avo/edit

Piston Links:
* https://www.reddit.com/r/rust_gamedev/
* https://github.com/PistonDevelopers/Piston-Tutorials
* https://github.com/pistondevelopers/piston-examples


Ihr habt den Umfang auch im Wesentlichen schon gut abgesteckt; also 
Kampfarena (vielleicht erstmal mit nur einem Modus und dann könnt ihr 
weitersehen ;)) mit 2D-Graphik -- vielleicht so wie in den frühen 
Pokémon-Spielen -- und einer einfachen KI sollte machbar sein. Alles 
darüber hinaus wäre natürlich super.

Wenn ihr noch keine Erfahrung mit OpenGL habt, würde ich euch von der 
direkten Benutzung von glium abraten, da man da schon recht viel Zeit 
verlieren kann, etwas auf den Bildschirm zu kriegen; und wenn man es 
dann geschafft hat, ist der Code ziemlich komplex und low-level, sodass 
es dann noch mehr Aufwand ist, das irgendwie hübsch zu verpacken, damit 
ihr euren Code auch noch versteht und die eigentlichen features drum 
herum bauen könnt. Stattdessen rate ich euch, euch mal piston [http://www.piston.rs] 
anzuschauen. Das ist eine Spiele-Engine in Rust, die schon Funktionen 
zum einfachen Zeichnen von 2D-Graphik eingebaut hat.

Es wäre sehr schade, wenn ihr viel Zeit für das händische Aufbauen der 
Pokémon-Datenbank "verschwenden" würdet. Da ihr ja aber irgendwoher 
Daten braucht, sollten wir auch das als Gelegenheit sehen, Rust zu 
schreiben. Es gibt diverse Webseiten wie Bulbapedia oder auch direkt 
Datenbanken/-sätze (wie den hier: https://github.com/veekun/pokedex), 
aus denen man diese Informationen natürlich auch automatisiert 
extrahieren kann, indem man z.B. die entsprechenden HTML-Seiten in einem 
Rust-Programm herunterlädt und die gesuchten Dinge herausparst oder eben 
direkt entsprechende Datenbanken anzapft. Dann müsst ihr euch auch nicht 
mehr auf die ersten 151 beschränken. ;)
