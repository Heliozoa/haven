module Pages.Login exposing (Model, Msg, page)

import Api exposing (..)
import Bindings exposing (..)
import Components.Section
import Effect exposing (Effect)
import Gen.Params.Login exposing (Params)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Page
import Request
import Shared
import View exposing (View)


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page shared req =
    Page.advanced
        { init = init shared
        , update = update req
        , view = view
        , subscriptions = subscriptions
        }



-- INIT


type alias Model =
    { email : String
    , password : String
    , status : Status
    }


type Status
    = None
    | Success
    | Error


init : Shared.Model -> ( Model, Effect Msg )
init _ =
    ( Model "" "" None
    , Effect.none
    )



-- UPDATE


type Msg
    = ClickedLogIn
    | SetEmail String
    | SetPassword String
    | GotUser (ApiResult User)


update : Request.With Params -> Msg -> Model -> ( Model, Effect Msg )
update _ msg model =
    case msg of
        ClickedLogIn ->
            ( { model | status = None }
            , Effect.fromCmd (login { email = model.email, password = model.password } GotUser)
            )

        SetEmail email ->
            ( { model | email = email }, Effect.none )

        SetPassword password ->
            ( { model | password = password }, Effect.none )

        GotUser (Ok user) ->
            ( Model "" "" Success
            , Effect.fromShared (Shared.LogIn user)
            )

        GotUser (Err _) ->
            ( { model | status = Error }, Effect.none )



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
                    div [] [ text "Logged in!" ]

                Error ->
                    div [] [ text "Failed to log in!" ]

                None ->
                    div [] []
    in
    { title = "Log In"
    , body =
        [ Components.Section.view []
            [ div [ class "field" ]
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
                    [ button [ class "button is-link", onClick ClickedLogIn ]
                        [ text "Submit" ]
                    ]
                ]
            , status
            , div []
                [ text "Forgot your password? Tough luck!" ]
            , div []
                [ text "Don't have an account yet? Register "
                , a [ href "/register" ] [ text "here" ]
                , text "."
                ]
            ]
        ]
    }
