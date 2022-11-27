module Components.Spinner exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Spinner


view : Spinner.Model -> Html msg
view spinner =
    let
        config =
            Spinner.defaultConfig
    in
    section
        [ style "z-index" "-1"
        , style "position" "static"
        ]
        [ Spinner.view { config | scale = 0.8 } spinner ]
