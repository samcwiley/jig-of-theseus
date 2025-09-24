\version "2.25.21"

\include "bagpipe.ly" 

\include "./music/includes/scw_bagpipe.ly"
\include "./music/includes/score_settings.ly"

source = "Trad."

#(allow-volta-hook "|")


voltaTwo = \markup  { \hspace #20 \italic \fontsize #+5 { "2" }  }
    
atholl_highlanders = {
	\time 6/8
	\repeat volta 2 {
		\grg e4. \gre e8 [ c8 \grd a8 ] |
		\grg e8 [ c8 \grd a8 ] \grg b8 [ c8 d8 ] |
		\grg e4. \gre e8 [ c8 \grd a8 ] |
		\grg b8 [ c8 d8 ] \grc c8 [ b8 \grG a8 ] |
		\break
		\grg e4. \gre e8 [ c8 \grd a8 ] |
		\grg e8 [ c8 \grd a8 ] \grg b8 [ c8 d8 ] |
		\grc c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 \gre b8 ] \grG a4. |
		\break
	}
	\repeat volta 2 {
		\grg a8 [ \grd c8 e8 ] \grg a8 [ \grd c8 e8 ] |
		\grg b8 [ d8 f8 ] \grg b8 [ d8 f8 ] |
		\grg a8 [ \grd c8 e8 ] \grg a8 [ \grd c8 e8 ] |
		\grg b8 [ c8 d8 ] \grc c8 [ b8 \grG a8 ] |
		\break
		\grg a8 [ \grd c8 e8 ] \grg a8 [ \grd c8 e8 ] |
		\grg b8 [ d8 f8 ] \grg b8 [ d8 f8 ] |
		\grc c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 \gre b8 ] \grG a4. |
		\break
	}
	\repeat volta 2 {
		A4 e8 ] \gre e8 [ d8 c8 ] |
		A8 [ e8 \gra e8 ] \gre e8 [ d8 c8 ] |
		A4 e8 ] \gre e8 [ d8 c8 ] |
		\grg b8 [ c8 d8 ] \grc c8 [ b8 \grG a8 ] |
		\break
		A8 [ e8 \gra e8 ] \gre e8 [ d8 c8 ] |
		A8 [ e8 \gra e8 ] \gre e4 d8 ] |
		\grc c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 \gre b8 ] \grG a4. |
		\break
	}
	\repeat volta 2 {
		\grc c8 [ a8 \grd c8 ] \grc c8 [ a8 \grd c8 ] |
		\grd d8 [ b8 d8 ] \grd d8 [ b8 d8 ] |
		\grc c8 [ a8 \grd c8 ] \grc c8 [ a8 \grd c8 ] |
		\grb b8 [ G8 \grd b8 ] \grg b8 [ c8 d8 ] |
		\break
		\grc c8 [ a8 \grd c8 ] \grc c8 [ a8 \grd c8 ] |
		\grd d8 [ b8 d8 ] \grg f8 [ e8 d8 ] |
		\grc c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 \gre b8 ] \grG a4. |
		\break
	}
}


\header { 
  title = \markup  \override #'(line-width . 82) 
  { 
    \column {  
      \center-align {
        \line { 
          Atholl Highlanders
        }
      }
    }
  }
                  
  subtitle = ""
  composer = "Trad."
  arranger = ""
  meter = "" 
}    


\paper {
	#(set-paper-size "letter" 'portrait)
}

\score {
	\new GrandStaff <<
		\new Staff = "GHB" <<
			\new Voice {
			        \global
				\atholl_highlanders
			}
		>>		
	>>
        \layout { \ScoreLayout 
                  \context { 
                             \Score
                             \override SpacingSpanner.base-shortest-duration = #(ly:make-moment 1/2) 
                           }            
                }
          
                \header{
        }
}
