module Pages.Post.Id_ exposing (Model, Msg, page)

import Api exposing (ApiResult)
import Bindings exposing (..)
import Browser.Events
import Components.CommentForm as CommentForm
import Components.Error
import Components.Section
import Components.Spinner
import Gen.Params.Post.Id_ exposing (Params)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode
import Page
import Request
import Set exposing (Set)
import Shared
import Spinner
import Url exposing (Protocol(..))
import View exposing (View)


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page shared req =
    Page.element
        { init = init req.params.id
        , update = update
        , view = view shared
        , subscriptions = subscriptions
        }



-- INIT


type Post
    = Loading Spinner.Model
    | Errored String
    | Loaded PostFull


type EditingCommentState
    = NotEditingComment
    | EditingComment Int String
    | SendingEditedComment Int String


type alias Model =
    { post : Post
    , comment : String
    , sendingComment : Bool
    , editingComment : EditingCommentState
    , deletingComments : Set Int
    , apiError : Maybe String
    , modal : Maybe Image
    }


init : String -> ( Model, Cmd Msg )
init id =
    ( { post = Loading Spinner.init
      , comment = ""
      , sendingComment = False
      , editingComment = NotEditingComment
      , deletingComments = Set.empty
      , apiError = Nothing
      , modal = Nothing
      }
    , Api.getPost id GotPost
    )



-- UPDATE


type Msg
    = GotPost (ApiResult PostFull)
    | TypedComment String
    | ClickedSubmitComment
    | GotComments (ApiResult (List Comment))
    | OpenModal Image
    | CloseModal
    | PressedKey
    | ClickedEditCommentButton Int String
    | ClickedDeleteCommentButton Int
    | ClickedSaveEditedCommentButton
    | ClickedCancelEditingCommentButton
    | TypedEditedComment String
    | Spin Spinner.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotPost (Ok post) ->
            ( { model | post = Loaded post }
            , Cmd.none
            )

        GotPost (Err err) ->
            ( { model | post = Errored err.message }, Cmd.none )

        TypedComment comment ->
            ( { model | comment = comment }, Cmd.none )

        ClickedSubmitComment ->
            case model.post of
                Loaded post ->
                    ( { model
                        | sendingComment = True
                        , apiError = Nothing
                      }
                    , Api.postComment post.id { text = model.comment } GotComments
                    )

                _ ->
                    ( model, Cmd.none )

        GotComments (Ok comments) ->
            case model.post of
                Loaded post ->
                    ( { model
                        | post = Loaded { post | comments = comments }
                        , comment = ""
                        , sendingComment = False
                        , apiError = Nothing
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        GotComments (Err err) ->
            ( { model
                | sendingComment = False
                , apiError = Just err.message
              }
            , Cmd.none
            )

        OpenModal image ->
            ( { model | modal = Just image }, Cmd.none )

        CloseModal ->
            ( { model | modal = Nothing }, Cmd.none )

        PressedKey ->
            ( model, Cmd.none )

        ClickedEditCommentButton id comment ->
            ( { model | editingComment = EditingComment id comment }, Cmd.none )

        ClickedDeleteCommentButton id ->
            case model.post of
                Loaded post ->
                    ( { model | deletingComments = Set.insert id model.deletingComments }, Api.deleteComment post.id id GotComments )

                _ ->
                    ( model, Cmd.none )

        ClickedSaveEditedCommentButton ->
            case ( model.post, model.editingComment ) of
                ( Loaded post, EditingComment id comment ) ->
                    ( model, Api.editComment post.id id { text = comment } GotComments )

                _ ->
                    ( model, Cmd.none )

        ClickedCancelEditingCommentButton ->
            ( { model | editingComment = NotEditingComment }, Cmd.none )

        TypedEditedComment comment ->
            case model.editingComment of
                EditingComment id _ ->
                    ( { model | editingComment = EditingComment id comment }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        Spin s ->
            case model.post of
                Loading spinner ->
                    ( { model | post = Loading (Spinner.update s spinner) }, Cmd.none )

                _ ->
                    ( model, Cmd.none )



-- SUBSCRIPTIONS


keyDecoder : Json.Decode.Decoder Msg
keyDecoder =
    Json.Decode.map
        (\s ->
            case s of
                "Escape" ->
                    CloseModal

                _ ->
                    PressedKey
        )
    <|
        Json.Decode.field "key" Json.Decode.string


subscriptions : Model -> Sub Msg
subscriptions model =
    case model.post of
        Loading _ ->
            Sub.map Spin Spinner.subscription

        _ ->
            case model.modal of
                Just _ ->
                    Browser.Events.onKeyDown keyDecoder

                _ ->
                    Sub.none



-- VIEW


viewImages : List Image -> Html Msg
viewImages images =
    div [ class "is-flex", style "justify-content" "center" ]
        (List.map
            (\image ->
                figure
                    [ style "width" "100%"
                    , style "max-width" (String.fromInt image.width ++ "px")
                    , style "aspect-ratio" (String.fromInt image.width ++ "/" ++ String.fromInt image.height)
                    , onClick (OpenModal image)
                    ]
                    [ img [ src (Api.imageSrc image.filename), style "object-fit" "scale-down", style "display" "block", attribute "loading" "lazy" ] []
                    ]
            )
            images
        )


viewTags : PostFull -> Html Msg
viewTags post =
    div []
        (List.map
            (\tag -> div [] [ text tag.aliasName ])
            post.tags
        )


viewPost : PostFull -> Html Msg
viewPost post =
    let
        userLink =
            "/user/" ++ String.fromInt post.userId
    in
    article [ class "media box" ]
        [ figure [ class "media-left" ]
            [ p [ class "image is-64x64" ]
                [ a [ href userLink ]
                    [ img [ src (Api.avatarSrc post.avatar), attribute "loading" "lazy" ] []
                    ]
                ]
            ]
        , div [ class "media-content" ]
            [ div [ class "content" ]
                [ div []
                    [ text post.title ]
                , div []
                    [ a [ href userLink ]
                        [ text post.username ]
                    ]
                , div [ style "white-space" "break-spaces" ]
                    [ text post.text ]
                ]
            ]
        ]


viewComment : Shared.Model -> EditingCommentState -> Comment -> Html Msg
viewComment shared editingComment comment =
    let
        editButton =
            case editingComment of
                NotEditingComment ->
                    case shared.user of
                        Just user ->
                            if user.id == comment.userId then
                                button [ class "button", onClick <| ClickedEditCommentButton comment.id comment.text ]
                                    [ text "Edit" ]

                            else
                                text ""

                        Nothing ->
                            text ""

                _ ->
                    text ""

        deleteButton =
            case editingComment of
                NotEditingComment ->
                    case shared.user of
                        Just user ->
                            if user.id == comment.userId then
                                button [ class "button", onClick <| ClickedDeleteCommentButton comment.id ]
                                    [ text "Delete" ]

                            else
                                text ""

                        Nothing ->
                            text ""

                _ ->
                    text ""

        saveButton =
            case editingComment of
                EditingComment _ _ ->
                    button [ class "button", onClick <| ClickedSaveEditedCommentButton ] [ text "Save" ]

                SendingEditedComment _ _ ->
                    button [ class "button", onClick <| ClickedSaveEditedCommentButton ] [ text "Save" ]

                _ ->
                    text ""

        cancelButton =
            case editingComment of
                EditingComment _ _ ->
                    button [ class "button", onClick <| ClickedCancelEditingCommentButton ] [ text "Cancel" ]

                SendingEditedComment _ _ ->
                    button [ class "button", disabled True ] [ text "Cancel" ]

                _ ->
                    text ""

        commentField =
            case editingComment of
                EditingComment id c ->
                    if id == comment.id then
                        textarea [ class "textarea", value c ] []

                    else
                        text comment.text

                SendingEditedComment id c ->
                    if id == comment.id then
                        textarea [ class "textarea", value c, disabled True ] []

                    else
                        text comment.text

                _ ->
                    text comment.text
    in
    article [ class "media box", style "margin-top" "1rem" ]
        [ figure [ class "media-left image is-64x64" ]
            [ a [ href <| "/user/" ++ String.fromInt comment.userId ]
                [ img [ src (Api.avatarSrc comment.avatar), attribute "loading" "lazy" ] []
                ]
            ]
        , div [ class "media-content" ]
            [ div [ class "content" ]
                [ div []
                    [ a [ href <| "/user/" ++ String.fromInt comment.userId ]
                        [ text comment.username ]
                    ]
                , div [ style "white-space" "break-spaces" ]
                    [ commentField ]
                ]
            ]
        , div [ class "media-right" ]
            [ editButton
            , deleteButton
            , saveButton
            , cancelButton
            ]
        ]


viewComments : Shared.Model -> EditingCommentState -> List Comment -> Html Msg
viewComments shared editingComment comments =
    div []
        (List.map
            (viewComment shared editingComment)
            comments
        )


viewModal : Maybe Image -> Html Msg
viewModal maybeImage =
    let
        active =
            case maybeImage of
                Just _ ->
                    "is-active"

                Nothing ->
                    ""

        fig =
            case maybeImage of
                Just image ->
                    [ figure
                        [ style "width" (String.fromInt image.width ++ "px")
                        , style "height" (String.fromInt image.height ++ "px")
                        ]
                        [ img [ src (Api.imageSrc image.filename), style "display" "block", attribute "loading" "lazy" ] []
                        ]
                    ]

                Nothing ->
                    []
    in
    div [ class "modal", class active ]
        [ div [ class "modal-background", onClick CloseModal ] []
        , div [ class "modal-content", style "width" "inherit" ]
            fig
        , button [ class "modal-close is-large", onClick CloseModal ] []
        ]


view : Shared.Model -> Model -> View Msg
view shared { post, comment, modal, sendingComment, editingComment } =
    let
        title =
            case post of
                Loaded p ->
                    p.title

                _ ->
                    ""

        body =
            case post of
                Loading spinner ->
                    [ Components.Section.view [] [ Components.Spinner.view spinner ] ]

                Errored err ->
                    [ Components.Section.view [] [ Components.Error.view <| "Failed to get post: " ++ err ] ]

                Loaded p ->
                    [ div []
                        [ viewImages p.images
                        , Components.Section.view []
                            [ viewTags p
                            , viewPost p
                            , CommentForm.view comment TypedComment ClickedSubmitComment sendingComment
                            , viewComments shared editingComment p.comments
                            , viewModal modal
                            ]
                        ]
                    ]
    in
    { title = title
    , body =
        body
    }
