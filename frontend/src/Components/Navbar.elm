module Components.Navbar exposing (..)

import Bindings exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)


view : Maybe User -> msg -> Html msg
view user logout =
    nav [ class "navbar is-transparent", style "background-color" "#e0e0e0" ]
        [ div [ class "navbar-brand container" ]
            [ div [ class "navbar-item" ]
                [ a [ class "navbar-item", href "/" ] [ text "Home" ]
                , case user of
                    Just _ ->
                        a [ class "navbar-item", href "/upload" ] [ text "Upload" ]

                    Nothing ->
                        text ""
                , case user of
                    Just _ ->
                        a [ class "navbar-item", onClick logout ] [ text "Log out" ]

                    Nothing ->
                        a [ class "navbar-item", href "/login" ] [ text "Log in" ]
                , case user of
                    Just u ->
                        a [ class "navbar-item", href ("/user/" ++ String.fromInt u.id) ] [ text "Profile" ]

                    Nothing ->
                        a [ class "navbar-item", href "/register" ] [ text "Register" ]
                ]
            ]
        ]
