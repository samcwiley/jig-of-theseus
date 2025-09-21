# Notes on Reading/Writing BMW

I cannot find a guide online for this format (`.bww`) file extensions, used by
Bagpipe Music Writer Gold and Bagpipe Player. These notes I have taken are based
on looking through many `.bww` files so may be incomplete (such as with grace
notes/embellishments that I could not find in any files)

## Notes

A note contains a pitch, a duration, an optional dot, and an optional bit for
explaining the direction of the beam for connected 8th, 16th, etc.

### Pitch

Pitches for melody notes are in uppercase. They are: `LG`, `LA`, `B`, `C`, `D`,
`E`, `F`, `HG`, `HA`

Pitches used in dots and embellishments are the same but in lowercase. They are:
`lg`, `la`, `b`, `c`, `d`, `e`, `f`, `hg`, `ha`

### Duration

Durations are numbers based on note values: `1` (Whole note), `2` (half note),
`4` (quarter note), `8` (eighth note), `16` (sixteenth note), `32`
(thirty-second note). They are provided after the note (and optional beaming)
and an underscore.

Example: a quarter note on D: `D_4`

### Dots

Dots must be manually placed in the place corresponding to their note. They use
a `'` followed by the lowercase representation of the pitch.

Example: a dotted quarter on low a: `LA_4 'la`

### Ties

Ties must be placed manually in the place corresponding to their note. They use
a `^t` followed by the lowercase represenation of the pitch.

Example: a quarter note on D tied to an eighth note on D: `D_4 ^td D_8`

### Beaming

Beaming is given by an `l` or `r` to denote which direction the beam is facing.
It comes after the pitch and before the note duration. The first note of a group
will have an `r` to denote that its beam connects it to a note to its right, and
the rest of the beamed notes in a group will have `l`.

Example: an 8th note on D, followed by a 16th note on D, followed by a 16th note
on F: `Dr_8 Er_16 Fl_16`

## Grace Notes and Embellishments

These are unique in that they are all lowercase with no other punctuation.

### Single Gracenotes

| Grace note     | BMW Representation(s) |
| -------------- | --------------------- |
| low g          | `strlg`               |
| low a          | `strla`               |
| b              | ?                     |
| c              | `strc`                |
| d              | `dg`                  |
| e              | `eg` or `stre`        |
| f              | `strf`                |
| high g         | `gg`                  |
| high a (thumb) | `tg`                  |

### Doublings

All of the doublings seem to be `db` followed by their respective lowercase
note.

Example: D doubling: `dbd`

### Thumb Doublings

Prepend the proper doubling with `t`

Example: Thumb doubling on F: `tdbf`

### Half Doublings

Prepend the proper doubling with `h`

Example: Half doubling on E: `hdbe`

### Strikes/Slurs/Shakes

(whatever you personally choose to call a g grace note to b, followed by a low g
strike on b)

| Embellishment | BMW     |
| ------------- | ------- |
| Low G         | X       |
| Low A         | ?       |
| B             | `gstb`  |
| C             | ?       |
| D (heavy)     | `lgstd` |
| D (light)     | ?       |
| E             | ?       |
| F             | ?       |
| High G        | ?       |
| High A        | `dblha` |

### Hornpipe Shakes/ Peles

(whatever you personally choose to call a D doubling followed by a d strike,
like on C in pumpkin's fancy)

Prepend `pel` with the appropriate lowercase pitch note.

Example: `pelc` for hornpipe shake on C

(This at least holds for `pelb` and `peld` as well, the rest of the notes I'm
unsure if they are valid as they are not commonly used in tunes)

### Other Embellishments

Some of these are rare or mostly used in piobaireachd, so I am having trouble
finding them used in tunes

| Embellishment       | BMW      |
| ------------------- | -------- |
| Grip                | `grp`    |
| Grip w/ b g.n.      | `grpb`   |
| Taorluath           | `taor`   |
| Taorluath w/ b g.n. | `taorb`  |
| Low g taorluath     | ?        |
| D throw             | `thrd`   |
| Crunluath           | `crunl`  |
| Crunluath w/ b g.n. | `crunlb` |
| Heavy Crunluath     | ?        |
| Heavy B Crunluath   | ?        |
| Edre                | `edre`   |
| Dare                | `dare`   |
| Chedari             | ?        |
| Embari              | ?        |
| Hodro               | `ggrpc`  |
| Hiotro              | `ggrpb`  |
| Birl w/ g g.n.      | `gbr`    |
| Birl from a         | `brl`    |
| Birl w/ low a       | `abr`    |
| Darodo              | `bubly`  |

## Constructing a measure

Measures start with `!`, then have the various grace notes and theme notes.

### Pickup measures

There is no measure validation for correct # of beats, so you can just include a
measure of whatever length before the first full measure starts

## Constructing a line

Each line starts with `& sharpf sharpc` (with an optional time signature if it's
the first part) and ends with either `!t` (line break?), `!I` (final bar line?),
or `''!I` (repeat).

### Time signatures

`2_4`, `6_8`, `4_4`, etc.

## Second Endings

First ending starts with `'1` and ends with `_'` Second ending simialrly starts
with `'2` and ends with `_'`

## Misc

`space` seems to be a keyword

## Constructing a tune

This block is at the top of every tune. Seems to be various metadata used for
display purposes.

```bww
Bagpipe Reader:1.0

MIDINoteMappings,(54,56,58,59,61,63,64,66,68,56,58,60,61,63,65,66,68,70,55,57,59,60,62,64,65,67,69)

FrequencyMappings,(370,415,466,494,554,622,659,740,831,415,466,523,554,622,699,740,831,932,392,440,494,523,587,659,699,784,880)

InstrumentMappings,(71,71,45,33,1000,60,70)

GracenoteDurations,(20,40,30,50,100,200,800,1200,250,250,250,500,200)

FontSizes,(100,100,65,70,300)

TuneTempo,60

TuneFormat,(1,1,F,L,500,500,500,500,L,0,0)

"Scotland The Brave",(T,L,0,0,Times New Roman,16,700,0,0,18,0,0,0)

"March",(Y,C,0,0,Times New Roman,14,400,0,0,18,0,0,0)

"Trad.",(M,R,0,0,Times New Roman,14,400,0,0,18,0,0,0)

"",(F,R,0,0,Times New Roman,10,400,0,0,18,0,0,0)

```
