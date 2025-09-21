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

### Half slurs (not currently supported)

Use `\hslur` with the proper note.

Example: Half slur on b: `\hslurb`

*Note:* Half slurs on low g and high a can not be played and will therefore not be supported.

### Thumb slurs (not currently supported)

Use `\tslur` with the proper note.

Example: Thumb slur on b: `\tslurb`

*Note:* Half slurs on low g and high a can not be played and will therefore not be supported.

### Catches (G grace note grips) (not currently supported)

A g grace note to a theme note, followed by a grip on that note. Use `\catch` with the proper note. These are supported for low a, b, c, d (with b grace note), and e.

Example: Catch on b: `\catchb`

### Half Catches (not currently supported)

A theme note, followed by a grip on that note. Use `\hcatch` with the proper note. These are supported for low a, b, c, d (with b grace note) and e.

Example: Half catch on c: `\hcatchc`

### Thumb catches (not currently supported)

Like a catch, but coming from high g, so it will have a high A (thumb) grace note instead of a g grace note. Supported for low a, b, c, d (with b grace note) and e. 

Example: Thumb catch on low a: `\tcatcha`

### Throws

| Embellishment | Lilypond | Currently Supported |
| ------------- | -------- | ------------------- |
| D throw       | `\thrwd` | ✓ |
| D throw from low G | `\Gthrwd` | X |
| Heavy d throw | `\gripthrwd` | X |
| Edre (e throw) | `\thrwe` or `\dre` | ✓ |
| Dare (f throw) | `\thrwf` or `\dare` | ✓ | 
| Bari (high g throw from low g) | `\bari` | ✓ |
| Chedari (high g through from e) | `\dari` | ✓ |
| D throw with long low g | `\pthrwd` | X |
| Darodo (Bubbly) | `\darodo` | ✓ |
| Darodo from low g | `\Gdarodo` | X |
| Darodo from low g w/ long final low G | `\pGdarodo` | X |

### Birls

| Embellishment | Lilypond | Currently Supported |
| ------------- | -------- | ------------------- |
| Birl on a  | `\wbirl` | ✓ |
| Birl w/ low a grace note | `\birl` | ✓ |
| Birl w/ g grace note | `\gbirl` | ✓ |
| Birl w/ d grace note | `\dbirl` | X |

### Grips

| Embellishment | Lilypond | Currently Supported |
| ------------- | -------- | ------------------- |
| Grip | `\grip` | ✓ | 
| Grip w/ b grace note | `\bgrip` | ✓ |
| Grip w/ e grace note (hadeda) | `\egrip` | X |

### Taorluaths

| Embellishment | Lilypond | Currently Supported |
| ------------- | -------- | ------------------- |
| Taorluath | `\taor` | ✓ |
| "Taorjmd"? Like a taorluath but the third grade note is a low a instead of low g | `\taorjmd` | X |
| Redundant low a taorluath | `\taorold` | X |
| Taorluath w/ b grace note | `\btaor` | ✓ |
| Taorluath from low g | `\Gtaor` | X |
| Taorluaths a mach | `\taoramb`, `\taoramc`, `\taoramd` | X |

### Crunluaths

| Embellishment | Lilypond | Currently Supported |
| ------------- | -------- | ------------------- |
| Crunluath | `\crun` | ✓ |
| Crunluath w/ b grace note | `\dcrun` | ✓ |
| Crunluath from low g | `\Gcrun` | ✓ |
| Crunluaths a mach | `\crunamb`, `\crunamc`, `\crunamd` | X |
| The second half of an amach (open edres) | `\crunambfosg`, `\crunamcfosg`, `\crunamdfosg` | X |

### "Weird stuff from Joseph MacDonald's Book

These are very long bagpipe embellishments, adapted from harp glissandos, and are not commonly used. I do not intend to support these at this time, but am including them in the documentation for completeness since they are included in `bagpipe.ly`.

| Embellishment | Lilypond | Currently Supported |
| ------------- | -------- | ------------------- |
| Fifteenth Cutting | `\fifteenthcutting` | X |
| Heavy Fifteenth Cutting | `\fifteenthcuttingG` | X |
| Fifteenth Cutting from low G | `\Gfifteenthcutting`| X |
| Heavy Fifteenth Cutting from low G | `\GfifteenthcuttingG` | X |
| Seventeenth Cutting | `\seventeenthcutting` | X |
| Heavy Seventeenth Cutting | `\seventeenthcuttingG` | X |
| Seventeenth Cutting from low G | `\Gseventeenthcutting` | X |
| Heavy Seventeenth Cutting from low G | `\GseventeenthcuttingG` | X |
| Barluadh (Barludh, Barluath) | `\barluadh` | X |
| Heavy Barluadh | `\barluadhG` | X |
| Barluadh from low G | `\Gbarluadh` | X |
| Heavy barluadh from low G | X |

### Other piobaireachd markup

I do not intend to support these at this time, but am including them in the documentation for completeness since they are included in `bagpipe.ly`.

| Embellishment | Lilypond | Currently Supported |
| ------------- | -------- | ------------------- |
| Trebling (three lines under note to denote GDE variation) | `\trebling` | X |
| Taorluath "T" under note | `\txtaor` | X |
| Crunluath "C" under note | `\txcrun` | X |
| Taorluath and Crunluath "T" and "C" under note | `\txtaorcrun` | X |
| Taorluath a Mach, upside down "T" under note | `\txtaoram` | X |
| Crunluath a Mach, backwards "C" under note | `\txcrunam` | X |
| Taorluath and Crunluath a Mach under note | `\txtaorcrunam` | X |