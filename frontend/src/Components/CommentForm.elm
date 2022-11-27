module Components.CommentForm exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)


view : String -> (String -> msg) -> msg -> Bool -> Html msg
view comment typeMsg clickMsg isDisabled =
    div []
        [ textarea
            [ class "textarea"
            , placeholder "Write a comment"
            , disabled isDisabled
            , rows 2
            , value comment
            , onInput typeMsg
            ]
            []
        , button [ class "button", onClick clickMsg, disabled isDisabled ]
            [ text "Submit" ]
        ]
