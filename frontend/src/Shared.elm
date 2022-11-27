module Shared exposing
    ( Flags
    , Model
    , Msg(..)
    , init
    , subscriptions
    , update
    , view
    )

import Bindings exposing (..)
import Components.Footer
import Components.Navbar
import Html exposing (..)
import Html.Attributes exposing (..)
import Json.Decode
import Ports
import Request exposing (Request)
import View exposing (View)


type alias Flags =
    Json.Decode.Value


type alias Model =
    { user : Maybe User
    }


init : Request -> Flags -> ( Model, Cmd Msg )
init _ flags =
    let
        user =
            Result.toMaybe (Json.Decode.decodeValue (Json.Decode.field "user" userDecoder) flags)
    in
    ( { user = user }
    , Cmd.none
    )


type Msg
    = LogIn User
    | LogOut


update : Request -> Msg -> Model -> ( Model, Cmd Msg )
update _ msg model =
    case msg of
        LogIn user ->
            ( { model | user = Just user }
            , Ports.storeUser user
            )

        LogOut ->
            ( { model | user = Nothing }
            , Ports.clearUser
            )


subscriptions : Request -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none


view :
    Request
    -> { page : View msg, logout : msg }
    -> Model
    -> View msg
view _ { page, logout } model =
    { title =
        if String.isEmpty page.title then
            "Haven"

        else
            page.title ++ " | Haven"
    , body =
        [ div [ class "is-flex is-flex-direction-column", style "min-height" "100vh" ]
            (Components.Navbar.view model.user logout
                :: page.body
                ++ [ Components.Footer.view ]
            )
        ]
    }
