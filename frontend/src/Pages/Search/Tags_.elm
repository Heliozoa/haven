module Pages.Search.Tags_ exposing (Model, Msg, page)

import Api exposing (..)
import Bindings exposing (..)
import Gen.Params.Search.Tags_ exposing (Params)
import Html exposing (..)
import Html.Attributes exposing (..)
import Page
import Request
import Shared
import View exposing (View)


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page _ req =
    Page.element
        { init = init (String.split "," req.params.tags)
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- INIT


type Model
    = Loading
    | Errored String
    | Loaded (List PostSmall)


init : List String -> ( Model, Cmd Msg )
init tags =
    ( Loading, findPosts tags GotPosts )



-- UPDATE


type Msg
    = GotPosts (ApiResult (List PostSmall))


update : Msg -> Model -> ( Model, Cmd Msg )
update msg _ =
    case msg of
        GotPosts (Err error) ->
            ( Errored (Debug.toString error), Cmd.none )

        GotPosts (Ok posts) ->
            ( Loaded posts, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> View Msg
view model =
    { title = "Search"
    , body =
        case model of
            Loading ->
                [ text "Loading..." ]

            Errored err ->
                [ text err ]

            Loaded [] ->
                [ text "No posts found" ]

            Loaded posts ->
                List.map (\p -> div [] [ text p.title ]) posts
    }
