module Pages.Register exposing (Model, Msg, page)

import Api exposing (..)
import Bindings exposing (..)
import Components.Section
import Gen.Params.Register exposing (Params)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Page
import Request
import Shared
import Url exposing (Protocol(..))
import View exposing (View)


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page _ _ =
    Page.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- INIT


type alias Model =
    { name : String
    , email : String
    , password : String
    , status : Status
    }


type Status
    = None
    | Success
    | Error


init : ( Model, Cmd Msg )
init =
    ( Model "" "" "" None, Cmd.none )



-- UPDATE


type Msg
    = ClickedRegister
    | SetName String
    | SetEmail String
    | SetPassword String
    | GotUser (ApiResult ())


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ClickedRegister ->
            ( Model "" "" "" None
            , Api.register
                { name = model.name, email = model.email, password = model.password }
                GotUser
            )

        SetName name ->
            ( { model | name = name }, Cmd.none )

        SetEmail email ->
            ( { model | email = email }, Cmd.none )

        SetPassword password ->
            ( { model | password = password }, Cmd.none )

        GotUser (Ok _) ->
            ( Model "" "" "" Success, Cmd.none )

        GotUser (Err _) ->
            ( Model "" "" "" Error, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> View Msg
view model =
    let
        status =
            case model.status of
                Success ->
                    div [] [ text "Registered!" ]

                Error ->
                    div [] [ text "Failed to register!" ]

                None ->
                    div [] []
    in
    { title = "Register"
    , body =
        [ Components.Section.view []
            [ div [ class "field" ]
                [ label [ class "label" ]
                    [ text "Name" ]
                , div [ class "control" ]
                    [ input [ class "input", type_ "text", placeholder "name", value model.name, onInput SetName ] [] ]
                ]
            , div [ class "field" ]
                [ label [ class "label" ]
                    [ text "Email" ]
                , div [ class "control" ]
                    [ input [ class "input", type_ "text", placeholder "email", value model.email, onInput SetEmail ] [] ]
                ]
            , div [ class "field" ]
                [ label [ class "label" ]
                    [ text "Password" ]
                , div [ class "control" ]
                    [ input [ class "input", type_ "password", placeholder "password", value model.password, onInput SetPassword ] [] ]
                ]
            , div [ class "field" ]
                [ div [ class "control" ]
                    [ button [ class "button is-link", onClick ClickedRegister ]
                        [ text "Submit" ]
                    ]
                ]
            , status
            ]
        ]
    }
