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
		\grg e4. \grg e8 [ c8 \grd a8 ] |
		\grg e8 [ c8 \grd a8 ] \grg b8 [ c8 d8 ] |
		\grg e4. \grg e8 [ c8 \grd a8 ] |
		\grg b8 [ c8 d8 ] \grg c8 [ b8 a8 ] |
		\break
		\grg e4. \grg e8 [ c8 \grd a8 ] |
		\grg e8 [ c8 \grd a8 ] \grg b8 [ c8 d8 ] |
		\grg c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 b8 ] \grg a4. |
		\break
	}
	\repeat volta 2 {
		\grg a8 [ \grd c8 e8 ] \grg a8 [ \grd c8 e8 ] |
		\grg b8 [ d8 f8 ] \grg b8 [ d8 f8 ] |
		\grg a8 [ \grd c8 e8 ] \grg a8 [ \grd c8 e8 ] |
		\grg b8 [ c8 d8 ] \grg c8 [ b8 a8 ] |
		\break
		\grg a8 [ \grd c8 e8 ] \grg a8 [ \grd c8 e8 ] |
		\grg b8 [ d8 f8 ] \grg b8 [ d8 f8 ] |
		\grg c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 b8 ] \grg a4. |
		\break
	}
	\repeat volta 2 {
		A8 [ e8 \grg e8 ] \grg e8 [ d8 c8 ] |
		A8 [ e8 \grg e8 ] \grg e8 [ d8 c8 ] |
		A8 [ e8 \grg e8 ] \grg e8 [ d8 c8 ] |
		\grg b8 [ c8 d8 ] \grg c8 [ b8 a8 ] |
		\break
		A8 [ e8 \grg e8 ] \grg e8 [ d8 c8 ] |
		A8 [ e8 \grg e8 ] \grg e8 [ d8 c8 ] |
		\grg c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 b8 ] \grg a4. |
		\break
	}
	\repeat volta 2 {
		\grg c8 [ a8 \grd c8 ] \grg c8 [ a8 \grd c8 ] |
		\grg d8 [ b8 d8 ] \grg d8 [ b8 d8 ] |
		\grg c8 [ a8 \grd c8 ] \grg c8 [ a8 \grd c8 ] |
		\grg b8 [ G8 b8 ] \grg b8 [ c8 d8 ] |
		\break
		\grg c8 [ a8 \grd c8 ] \grg c8 [ a8 \grd c8 ] |
		\grg d8 [ b8 d8 ] \grg d8 [ b8 d8 ] |
		\grg c8 [ A8 e8 ] \grg f8 [ e8 d8 ] |
		\grg c8 [ d8 b8 ] \grg a4. |
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
