module Components.Section exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)


view : List (Attribute msg) -> List (Html msg) -> Html msg
view attrs htmls =
    section
        ([ class "section is-flex is-flex-grow-1", style "min-height" "10rem" ]
            ++ attrs
        )
        [ main_ [ class "container" ]
            htmls
        ]
