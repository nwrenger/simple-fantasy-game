# Simple Fantasy Game

Das `Simple Fantasy Game` emuliert ein Angriff zwischen einem Spieler und einem Gegner. Die Terminal eingabe Library ist dabei komplett selbst entwickelt und kann [hier](https://github.com/nwrenger/console-utils-rs) gefunden werden. Alles ist hauptsächlich selbst geschrieben, an den Stellen, wo jedoch ChatGPT verwendet wurde, wird es erwähnt.

In folgendem wird die generelle Struktur durch ein UML, die Implementation dieser in Rust und wie man das Programm startet erklärt.

## Generelle Struktur

![UML](./Simple%20Fantasy%20Game.drawio.svg)

## Implementation in Rust

> ChatGPT wurde hier in Teilen genutzt, um verständlichere Erklärungen zu schreiben.

In Rust wurde dies in großen Teilen sehr passend zum UML implementiert.

Die Vererbung wurde mit einem `Combatant` Trait und einem `Entity` Struct umgesetzt. Dabei ist ein **Trait** in Rust im Wesentlichen ein Sammlungsschema von Methoden und zugehörigen (ggf. Standard-)Implementationen, den ein Typ erfüllen muss. Methoden können dabei auch überschrieben werden (Polyphormie). Die `Monster`, `Fighter` und `Mage` Structs haben dabei diesen Trait implementiert und jeweils als ein Attribut `entity` mit dem Typ `Entity` (Struct).

Ein **Struct** ist dabei ein benutzerdefinierter Datentyp, mit welchem man mehrere Werte (Felder) logisch zusammenfassen kann und man für diesen Datentyp aufrufbare Methoden definieren kann.

Über einigen Structs steht dabei auch `#[derive(...)]`. Dies ist ein Attribut, womit Rust signalisiert wird, welche bestimmten Standard-Implementierungen („Ableitungen“), wie z.B. Kopieren (Clone) eines Structs, generiert werden sollen. Sie sind sehr ähnlich zu Dekoratoren in Python.

## Verwendung

```bash
./simple-fantasy-game [PFAD]
```

Der Pfad für die Konfigurationsdatei muss angeben werden. Wenn noch keine Konfigurationsdatei existiert wird eine erstellt. Hier sind zwei Bespiele für eine solche Datei:

```json
{
	"player": {
		"Fighter": {
			"entity": {
				"name": "Mario",
				"life_points": 120,
				"dexterity": 20,
				"strength": 30,
				"weapon": {
					"material": "Iron",
					"spell_power": 0
				}
			},
			"endurance": 4
		}
	},
	"enemy": {
		"entity": {
			"name": "Höllenhund",
			"life_points": 200,
			"dexterity": 10,
			"strength": 20,
			"weapon": null
		}
	}
}
```

```json
{
	"player": {
		"Mage": {
			"entity": {
				"name": "Alice",
				"life_points": 100,
				"dexterity": 12,
				"strength": 30,
				"weapon": {
					"material": "Wood",
					"spell_power": 4
				}
			},
			"magic_power": 14
		}
	},
	"enemy": {
		"entity": {
			"name": "Höllenhund",
			"life_points": 200,
			"dexterity": 10,
			"strength": 20,
			"weapon": null
		}
	}
}
```