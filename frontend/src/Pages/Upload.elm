module Pages.Upload exposing (Model, Msg, page)

import Api exposing (..)
import Bindings exposing (..)
import Components.Section
import File exposing (File)
import File.Select
import Gen.Params.Upload exposing (Params)
import Gen.Route
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Page
import Request
import Shared
import Task
import View exposing (View)


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page _ req =
    Page.protected.element
        (\_ ->
            { init = init
            , update = update req
            , view = view
            , subscriptions = subscriptions
            }
        )



-- INIT


type alias Model =
    { title : String
    , post : String
    , tags : List Int
    , images : List File
    , imageUrls : List String
    , search : String
    , status : Status
    }


type Status
    = None
    | Error String
    | Success


init : ( Model, Cmd Msg )
init =
    ( { title = ""
      , post = ""
      , tags = []
      , images = []
      , imageUrls = []
      , search = ""
      , status = None
      }
    , Cmd.none
    )



-- UPDATE


type Msg
    = TypedTitle String
    | TypedPost String
    | TypedTagSearch String
    | SetTags (List Int)
    | SelectImages
    | SelectedImages File (List File)
    | SetImageUrls (List String)
    | ClickedSubmit
    | GotResponse (ApiResult String)


update : Request.With Params -> Msg -> Model -> ( Model, Cmd Msg )
update req msg model =
    case msg of
        TypedTitle title ->
            ( { model | title = title }, Cmd.none )

        TypedPost title ->
            ( { model | post = title }, Cmd.none )

        TypedTagSearch search ->
            ( { model | search = search }, Cmd.none )

        SetTags tags ->
            ( { model | tags = tags }, Cmd.none )

        SelectImages ->
            ( model
            , File.Select.files [ "image/png", "image/jpg" ] SelectedImages
            )

        SelectedImages image images ->
            ( { model | images = image :: images, imageUrls = [] }
            , Task.perform SetImageUrls (Task.sequence (List.map File.toUrl (image :: images)))
            )

        SetImageUrls urls ->
            ( { model | imageUrls = urls }, Cmd.none )

        ClickedSubmit ->
            ( model
            , uploadPost
                { metadata =
                    { title = model.title
                    , post = model.post
                    , tags = model.tags
                    , thumbnail = { idx = 0, x = 0, y = 0, size = 99999 }
                    , nsfw = False
                    }
                , images = model.images
                }
                GotResponse
            )

        GotResponse (Ok id) ->
            ( { model | status = Success }, Request.pushRoute (Gen.Route.Post__Id_ { id = id }) req )

        GotResponse (Err err) ->
            ( { model | status = Error err.message }, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> View Msg
view model =
    let
        previews =
            if List.isEmpty model.images then
                div [] [ text "No images selected" ]

            else if List.isEmpty model.imageUrls then
                div [] [ text "Loading previews..." ]

            else
                div []
                    (List.map
                        (\url ->
                            img [ src url, style "object-fit" "cover", style "width" "10rem", style "height" "10rem", attribute "loading" "lazy" ] []
                        )
                        model.imageUrls
                    )

        status =
            case model.status of
                None ->
                    text ""

                Success ->
                    div [] [ text "Upload was successful!" ]

                Error err ->
                    div [] [ text <| "Failed to upload: " ++ err ]
    in
    { title = "Upload"
    , body =
        [ Components.Section.view []
            [ h1 [ class "title" ] [ text "Upload" ]
            , label [ for "title", class "label" ] [ text "Title" ]
            , input [ id "title", value model.title, onInput TypedTitle, class "input", placeholder "title" ] []
            , label [ for "post", class "label" ] [ text "Post" ]
            , textarea [ id "post", onInput TypedPost, class "textarea", placeholder "post" ] []
            , label [ for "tags", type_ "search", class "label" ] [ text "Tags" ]
            , input [ id "tags", class "input", placeholder "tags" ] []
            , label [ for "images", class "button" ] [ text "Choose images" ]
            , input
                [ id "images"
                , type_ "file"
                , multiple True
                , accept "png|jpeg"
                , onClick SelectImages
                , hidden True
                ]
                []
            , br [] []
            , previews
            , input [ type_ "submit", value "Submit", onClick ClickedSubmit, class "button" ] []
            , status
            ]
        ]
    }
