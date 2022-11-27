module Components.Error exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)


view : String -> Html msg
view message =
    section []
        [ text message ]
