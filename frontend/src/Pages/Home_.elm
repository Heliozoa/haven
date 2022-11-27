module Pages.Home_ exposing (Model, Msg, page)

import Api exposing (ApiResult)
import Bindings exposing (..)
import Components.Error
import Components.Section
import Components.Spinner
import Html exposing (..)
import Html.Attributes exposing (..)
import Page
import Request exposing (Request)
import Shared
import Spinner
import View exposing (View)


page : Shared.Model -> Request -> Page.With Model Msg
page _ _ =
    Page.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- INIT


type Model
    = Loading Spinner.Model
    | Errored String
    | Loaded (List PostSmall)


init : ( Model, Cmd Msg )
init =
    ( Loading Spinner.init
    , Api.getPosts GotPosts
    )



-- UPDATE


type Msg
    = GotPosts (ApiResult (List PostSmall))
    | SpinnerMsg Spinner.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotPosts (Ok posts) ->
            ( Loaded posts, Cmd.none )

        GotPosts (Err error) ->
            ( Errored error.message, Cmd.none )

        SpinnerMsg smsg ->
            case model of
                Loading spinner ->
                    ( Loading (Spinner.update smsg spinner), Cmd.none )

                _ ->
                    ( model, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    case model of
        Loading _ ->
            Sub.map SpinnerMsg Spinner.subscription

        _ ->
            Sub.none



-- VIEW


view : Model -> View Msg
view model =
    let
        content =
            case model of
                Loading spinner ->
                    Components.Spinner.view spinner

                Errored err ->
                    Components.Error.view <| "Failed to get posts: " ++ err

                Loaded [] ->
                    text "No posts found"

                Loaded posts ->
                    div [ class "is-flex is-flex-direction-row is-flex-wrap-wrap" ] <|
                        List.map
                            (\p ->
                                div [ class "m-2" ]
                                    [ viewPostSmall p ]
                            )
                            posts
    in
    { title = ""
    , body =
        [ Components.Section.view []
            [ h1 [ class "title" ]
                [ text "Haven" ]
            , content
            ]
        ]
    }


viewPostSmall : PostSmall -> Html Msg
viewPostSmall postSmall =
    div [ class "card", style "width" "10rem" ]
        [ div [ class "card-image" ]
            [ a [ href ("/post/" ++ String.fromInt postSmall.id) ]
                [ figure []
                    [ img [ src (Api.thumbSrc postSmall.thumbnail), style "object-fit" "cover", style "width" "10rem", style "height" "10rem" ] []
                    ]
                ]
            ]
        , div [ class "card-content p-1" ]
            [ div [ class "content" ]
                [ a [ href ("/post/" ++ String.fromInt postSmall.id) ]
                    [ p [ class "m-1", style "overflow" "hidden", style "white-space" "nowrap", style "text-overflow" "ellipsis" ]
                        [ text postSmall.title ]
                    ]
                , a [ href ("/user/" ++ String.fromInt postSmall.userId) ]
                    [ p [ class "m-1", style "overflow" "hidden", style "white-space" "nowrap", style "text-overflow" "ellipsis" ]
                        [ text postSmall.username ]
                    ]
                ]
            ]
        ]
