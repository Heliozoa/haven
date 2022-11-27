module Pages.Tag.Id_ exposing (Model, Msg, page)

import Api exposing (..)
import Bindings exposing (..)
import Gen.Params.Tag.Id_ exposing (Params)
import Html exposing (..)
import Html.Attributes exposing (..)
import Page
import Request
import Shared
import View exposing (View)


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page _ req =
    Page.element
        { init = init req.params.id
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- INIT


type Model
    = Loading
    | Errored String
    | Loaded Tag


init : String -> ( Model, Cmd Msg )
init id =
    ( Loading, getTag id GotTag )



-- UPDATE


type Msg
    = GotTag (ApiResult Tag)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg _ =
    case msg of
        GotTag (Err error) ->
            ( Errored (Debug.toString error), Cmd.none )

        GotTag (Ok tag) ->
            ( Loaded tag, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> View Msg
view model =
    { title = "Tag"
    , body =
        case model of
            Loading ->
                [ text "Loading..." ]

            Errored err ->
                [ text err ]

            Loaded tag ->
                [ text tag.alias ]
    }
