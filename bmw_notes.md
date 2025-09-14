# Notes on Reading/Writing BMW

I cannot find a guide online for this format (`.bww`) file extensions, used by Bagpipe Music Writer Gold and Bagpipe Player. These notes I have taken are based on looking through many `.bww` files so may be incomplete (such as with grace notes/embellishments that I could not find in any files)

## Notes
A note contains a pitch, a duration, an optional dot, and an optional bit for explaining the direction of the beam for connected 8th, 16th, etc.

### Pitch
Pitches for melody notes are in uppercase. They are:
`LG`, `LA`, `B`, `C`, `D`, `E`, `F`, `HG`, `HA`

Pitches used in dots and embellishments are the same but in lowercase. They are:
`lg`, `la`, `b`, `c`, `d`, `e`, `f`, `hg`, `ha`

### Duration
Durations are numbers based on note values:
`1` (Whole note), `2` (half note), `4` (quarter note), `8` (eighth note), `16` (sixteenth note), `32` (thirty-second note). They are provided after the note (and optional beaming) and an underscore.

Example: a quarter note on D: `D_4`

### Dots

Dots must be manually placed in the place corresponding to their note. They use a `'` followed by the lowercase representation of the pitch.

Example: a dotted quarter on low a: `LA_4 'la`

### Ties

Ties must be placed manually in the place corresponding to their note. They use a `^t` followed by the lowercase represenation of the pitch.

Example: a quarter note on D tied to an eighth note on D: `D_4 ^td D_8`

### Beaming

Beaming is given by an `l` or `r` to denote which direction the beam is facing. It comes after the pitch and before the note duration. The first note of a group will have an `r` to denote that its beam connects it to a note to its right, and the rest of the beamed notes in a group will have `l`. 

Example: an 8th note on D, followed by a 16th note on D, followed by a 16th note on F: `Dr_8 Er_16 Fl_16`



## Grace Notes and Embellishments

### Single Gracenotes

| Grace note | BMW Representation(s) |
| --------- | ---- |
| low g     | `strlg` |
| low a | `strla` |
| b | ? | 
| c | `strc` |
| d | `dg` |
| e         | `eg` or `stre` |
| f | `strf` |
| high g    | `gg` |
| high a (thumb) | `tg` |

### Doublings

All of the doublings seem to be `db` followed by their 

### Low g Strike D

Low g strike on D (like in amazing grace and Jig of slurs) `lgstd`