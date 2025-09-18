# Notes on Reading/Writing lilypond

[Lilypond](https://lilypond.org/) is a sheet music file format that is not specific to bagpipe music, but thanks to the [`bagpipe.ly`](https://lilypond.org/doc/v2.25/Documentation/notation/bagpipes.html) by [Sven Axelsson](http://svenax.net/site/category/music/) there are many additional reserved keywords for representing bagpipe embellishments, as well as shorthands for representing ontes on the bagpipe scale.

## Notes

A note contains a pitch, a duration, and an optional dot.

### Pitch

Pitches for melody notes involve the letter for the note they represent, mostly in lowercase, but with capital `G` representing low g, and capital `A` representing high a. 

This leaves the scale at:
`G`, `a`, `b`, `c`, `d`, `e`, `f`, `g`, `A`

### Duration

Durations are numbers based on note values:
`1` (Whole note), `2` (half note), `4` (quarter note), `8` (eighth note), `16` (sixteenth note), `32` (thirty-second note).

They are provided directly after the pitch in a note.

Example: a quarter note on D: `d4`

### Dots

A dot is represented simply by adding a `.` after a duration.

Example: a dotted quarter note on low a: `a4.`

### Ties

Ties are done using the `~` symbol, which connects either a pitch+duration with a duration (inferred to be on the same note), but it can be good practice to use it to connect a pitch+duration with another pitch+duration.

Example:a quarter note on D tied to an eighth note on d: can be represented as `d4~8` or `d4~d8`. 

*Note* `pibroxide` can handle reading either of these cases, but will output music using the latter (redundant, but explicit) case

### Beaming

Beaming is done by using square braces (`[` and `]`) to surround all notes that should be beamed in a group following the first note in the group. Lilypond can infer the beams, but it can be good practice to specify the beams manually.

Example: an 8th note on d, followed by a 16 note on e, followed by a 16th note on f: `d8 [ e16 f16 ]`. 

If grace notes are used in any of the beamed notes after the first, these should also be included in the brackets. 

Example: a 16th note on e, followed by a dotted 8th on low a with a g grace note, followed by a c with a d grace note: `e16 [\grg a8. \grd c8]`

## Grace Notes and Embellishments

These are all escaped macros starting with a `\` character. 

### Single Gracenotes

Simply use `\gr` to signify a grace note, followed by the corresponding letter to designate the pitch. This gives a grace note scale: `\grG`, `\gra`, `\grb`, `\grc`, `\grd`, `\gre`, `\grf`, `\grg`, `\grA`.

### Doublings

Like single gracenotes, all doublings are `\dbl` followed by their respective pitch.

Example: D doubling: `\dbld`

### Thumb Doublings

Prepend the proper doubling with `t` after the escape character.

Example: Thumb doubling on F: `\tdblf`

*Note:* thumb doublings on High A are unavailable.

### Half Doublings

Prepend the proper doubling with `h` after the escape character.

Example: Half doubling on e: `\hdble`.

*Note:* for high g and high a, the half doubling is equivalent to the normal doubling. Thus `\hdblg` is equivalent to `\dblg` as is `\hdblA` equivalent to `\dblA`.

### Slurs

Use `\slur` with the proper note.

Example: Slur on b: `\slurb`

*Note:* Slurs on low g, high g, and high a can not be played and will therefore not be supported.

### Half slurs

Use `\hslur` with the proper note.

Example: Half slur on b: `\hslurb`

*Note:* Half slurs on low g and high a can not be played and will therefore not be supported.

### Thumb slurs

Use `\tslur` with the proper note.

Example: Thumb slur on b: `\tslurb`

*Note:* Half slurs on low g and high a can not be played and will therefore not be supported.

### Catches (G grace note grips)

Use `\catch` with the proper note. These are supported for low a, b, c, d (with b grace note), and e.

Example: Catch on c: `\catchc`

