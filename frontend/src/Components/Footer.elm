module Components.Footer exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)


view : Html msg
view =
    footer [ class "footer" ]
        [ div [ class "container" ]
            [ a [ class "px-2", href "https://github.com/Heliozoa/haven" ]
                [ img [ src "/gh.png", alt "Find Haven's source code at GitHub", style "height" "32px", attribute "loading" "lazy" ] [] ]
            , a [ class "px-2", href "https://bulma.io" ]
                [ img [ src "https://bulma.io/images/made-with-bulma.png", alt "Made with Bulma", style "height" "32px", attribute "loading" "lazy" ] [] ]
            ]
        ]
