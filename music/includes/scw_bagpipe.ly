% ************************************************************************************
%
%	scw_bagpipe.ly: a place to put all the customizations that I created to 
%			address all the items that are missing in bagpipe.ly.
%
%			See also the score_settings.ly file.
%
%	Created by:	Sam Wiley, forked from Ben Beeson
%
%
%       Revised:		14 Dec 24 
%
%       Added \pickup command for better formatting than \partials
%
% ************************************************************************************
\version "2.25.4"
\include "bagpipe.ly"



global = {
	% bagpipe music is written in D major
	\key d \major
	% for historical reasons, pipers leave the key signature off the sheet music
	\hideKeySignature
	\numericTimeSignature
	% I like to have the bar numbers printed on the tunes
	\override  Score.BarNumber.break-visibility = ##(#t #t #t)
	\set Score.barNumberVisibility = #all-bar-numbers-visible
	\override  Score.BarNumber.break-visibility = #end-of-line-invisible
        % The next line is an example of how to raise the fermata a bit higher on the music score.
        \override Script.padding =#3 
        \override TupletBracket.bracket-visibility = ##t
        \override TupletBracket.padding = #4

}

pickup =
#(define-music-function (duration music) (ly:duration? ly:music?)
   #{ 
     \once \override Score.BarNumber.stencil = ##f
     \applyContext #(lambda (c)
       (let* ((timing-context (ly:context-find c 'Timing))
              (current-bar-number (ly:context-property timing-context 'currentBarNumber)))
         (if (> current-bar-number 1)
             (set! (ly:context-property timing-context 'currentBarNumber)
                   (1- current-bar-number)))))
     \partial #duration #music
   #})

%
% Here are some formatting tricks for barlines at the end of tune parts 
% 
%  

#(define-bar-line "||-b" "||" "||" "||") %This is for putting all parts in one score!
% See the CabarFeidh_Strathspey_Portrait.ly for an example.

#(define-bar-line "|.-b" "|." ".|" "|.") %This is for putting all parts in one score!
#(define-bar-line ":|.-.|" ":|." ".|" "|." )    
\defineBarLine    "|.-.|:" #'("|." ".|:" "" ) %This rocks! This ends a repeated part and starts a non-repeated part.
\defineBarLine    ":|.-.|:" #'(":|." ".|:" "" ) %This rocks! This ends a repeated part and starts a repeated part.
#(define-bar-line ":|.-.|" ":|." ".|" "|." )    

%
%  A fix to allow a volta bracket to have a closing hook.
%

#(allow-volta-hook "||")
#(allow-volta-hook "|")


%
% A fix to place certain text in a volta bracket -- usually used for piobaireachd notation in the middle of a line
%
voltaTwice = \markup  { \hspace #20 \italic \fontsize #+5 { Play Twice }  }
voltaLineTwoOnly = \markup { \hspace #15 \italic \fontsize #+5 { Line 2 Only } }
voltaLineThreeOnly = \markup { \hspace #15 \italic \fontsize #+5 { Line 3 Only } }
voltaSingling = \markup { \hspace #15 \italic \fontsize #+5 { S } }
voltaDoubling = \markup { \hspace #15 \italic \fontsize #+5 { D } }

% Cadence 'cad' is copied from bagpipe.ly and a fermata is added over the e to match the setting
cad_eye      = { \pgrace { \stemspace #'(0 . 0.5) g32[ e8\fermata d32] } } 


hiharin = { \grg e4 \dbirl a4 } 

 
 
%  define a g-e doubling for hornpipe music
gdedbl = { \grace { \small g32 [ d e ] }}
gbedbl = { \grace { \small g32 [ b e ] }}
gcedbl = { \grace { \small g32 [ c e ] }}



% other embellishments
gdcdbl =  { \pgrace { g32 [ d c ] } }   % { \grace { \small g32 [d c] }} % for example, Seige of Delhi 2nd part, bar 2
litedstrike = { \pgrace { d32 [c] }}

% funky stuff from Fred Morrison's settings
hthrwf =  { \grace { \small g32 [ e ]   }}
gcdbl =  { \grace { \small g32 [ c ]   }}

%doubling strike -- see McGillivray pg 75-78
% aka pella
slurcgrip =   { \grace { \small g32 [ c G d G ] }}
hslurcgrip =  { \grace { \small c32 [ G d G ]   }}


% crunluath on low A only -- I saw this once in light music, 
% I probably should rename this to adre or some variant, but
% since the d in adre is longer.... what to do? 
Acrun = { \grace { \small d32 [ a e a f a ] }}

%closed crunluath from b or low g
bamdre  = { \pgrace { G32[ d G e G f G ] } }


% insert a gracenote space and hide it.... 
% this is useful to align parts in a grand score if the parts 
% don't all start with a gracenote and at least one part does.
grghide = {\hideNotes { \pgrace { \small g32 }} \unHideNotes}
hdblhide = {\hideNotes { \pgrace { \small g32 [ d ] }} \unHideNotes}
dblhide = {\hideNotes { \pgrace { \small g32 [ d e ] }} \unHideNotes}
thrwdhide = {\hideNotes { \pgrace { \small G32 [ d c ] }} \unHideNotes}

% You can also use \grace \skip 32
% See https://lilypond.org/doc/v2.23/Documentation/notation/special-rhythmic-concerns 
% and refer to known issues and warnings at the bottom of the page for 
% a discussion on \grace \skip timevalue to fix the spacing issues!


% a half doubling on C with e-d gracenotes -- see Kilworth Hills part 3 
% from the Pipe Teacher's Assistant Vol 1. 
edblc = { \pgrace { \small e32[ d] } }


gdre  =  { \pgrace { g32 e32 [ a f a] } }

darem =  { \pgrace { G32 [ d32 G e ] } }


% ************* for darodo exercise *************
% ** this just takes a gracenote off so you can expand the embellishment ****
halfDarodo = { \pgrace { d32[ G c ] } }


%***********************  some markup stuff *****************

% For the downArrow -- adjust the horzontal spacing at the \hspace line below
downArrow= \mark \markup {
                                        \fontsize #5
                                        \override #'(thickness . 2)
                                        \hspace #6
                                        \combine
                                        \draw-line #'(0 . 4)
                                        \arrow-head #Y #DOWN ##f                                         
                        }
     
repeatAtStartOfPiece_new = {
                %for version 2.18 +
                \bar ".|:"
                \mark \default
}
     
     
repeatAtStartOfPiece = { 
  % see http://lilypond.org/doc/v2.14/Documentation/notation/long-repeats 
  % this is how you place the repeat inside the time signature on a staff.
                 \once \override Score.BreakAlignment.break-align-orders =
                  #(make-vector 3 '(instrument-name
                                    left-edge
                                    ambitus
                                    breathing-sign
                                    clef
                                    key-signature
                                    time-signature
                                    staff-bar
                                    custos))
                 \once \override Staff.TimeSignature.space-alist =
                               #'((time-signature extra-space . 1.15) 
                                  (staff-bar extra-space . 1.1) 
                                  (cue-clef extra-space . 0.5) 
                                  (left-edge extra-space . 0.0)
                                  (right-edge extra-space . 0.5) 
                                  (extra-spacing-height '(+inf.0 . -inf.0))
                                  (first-note fixed-space . 2.5)
                                  
                               
                                  ;;(first-note . (fixed-space . 2.0))
                                  ;;(right-edge . (extra-space . 0.5))
                                  ;; free up some space between time signature
                                  ;; and repeat bar line
                                  ;;(staff-bar . (extra-space . 1))
                                  )
                               \bar "|:"
}


% Abbreviated notation common in piobaireachd scores.
% TODO: Make sure these are put on a fixed Y-position.


txtaortaoramcruncrunam =  \markup { 
      \override #'(baseline-skip . 1.0)
      \column {
            \center-align "T"
            \center-align \scale #'(-1 . -1) "T"
            \lower #2.5 \center-align "C"
            \lower #1 \center-align \scale #'(-1 . -1) "C"
      }
}
  

txtaorcruncrunam =  \markup { 
      \override #'(baseline-skip . 1.0 )
      \column {
            \center-align \scale #'(1 . 1) "T"
            \center-align \scale #'(1 . 1) "C"
            %\lower #1 \center-align \scale #'(-1 . -1) "C"  
            \center-align \scale #'(-1 . -1) "C"
      }
}
     
        
% a high g throw that follows a full f note -- see example at The MacDougalls' Gathering, Kilberry Book of Ceol Mor, pg 48 urlar line 3, bar 5.
% note this is slightly different from the chedari at line 3, second bar since this one starts on e, not f. 
chedari = { \pgrace { e32[ g e f e] } }
