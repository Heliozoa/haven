module Pages.User.Id_ exposing (Model, Msg, page)

import Api exposing (ApiResult)
import Bindings exposing (..)
import Components.Error
import Components.Section
import Components.Spinner
import File exposing (File)
import File.Select
import Gen.Params.User.Id_ exposing (Params)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Page
import Request
import Shared
import Spinner
import Task
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


type User
    = Loading Spinner.Model
    | Errored String
    | Loaded UserView


type ProfileState
    = NotEditingProfile
    | EditingProfile { newName : String, newProfile : String }
    | SentEditedProfile { newName : String, newProfile : String }


type AvatarState
    = NotEditingAvatar
    | EditingAvatar File
    | EditingAvatarUrl File String
    | SentEditedAvatar File String


type FollowState
    = Idle
    | Attempting


type alias Model =
    { user : User
    , profileState : ProfileState
    , avatarState : AvatarState
    , followState : FollowState
    , apiError : Maybe String
    }


init : String -> ( Model, Cmd Msg )
init id =
    ( { user = Loading Spinner.init
      , profileState = NotEditingProfile
      , avatarState = NotEditingAvatar
      , followState = Idle
      , apiError = Nothing
      }
    , Api.getUser id GotUser
    )



-- UPDATE


type Msg
    = GotUser (ApiResult UserView)
    | ClickedFollow
    | GotFollow (ApiResult ())
    | ClickedUnfollow
    | GotUnfollow (ApiResult ())
    | ClickedEditProfile
    | ClickedSaveEditedProfile
    | GotEditedProfile (ApiResult ())
    | ClickedCancelEditingProfile
    | TypedName String
    | TypedProfile String
    | ClickedChangeAvatar
    | SelectedNewAvatar File
    | SetNewAvatarUrl String
    | ClickedSaveAvatar
    | GotNewAvatar (ApiResult String)
    | Spin Spinner.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotUser (Ok user) ->
            ( { model | user = Loaded user, apiError = Nothing }, Cmd.none )

        GotUser (Err err) ->
            ( { model | user = Errored err.message }, Cmd.none )

        ClickedFollow ->
            case model.user of
                Loaded user ->
                    ( { model
                        | followState = Attempting
                        , apiError = Nothing
                      }
                    , Api.follow user.id GotFollow
                    )

                _ ->
                    ( model, Cmd.none )

        GotFollow (Ok ()) ->
            case model.user of
                Loaded user ->
                    ( { model
                        | user = Loaded { user | following = True }
                        , followState = Idle
                        , apiError = Nothing
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        GotFollow (Err err) ->
            ( { model
                | followState = Idle
                , apiError = Just err.message
              }
            , Cmd.none
            )

        ClickedUnfollow ->
            case model.user of
                Loaded user ->
                    ( { model
                        | followState = Attempting
                        , apiError = Nothing
                      }
                    , Api.unfollow user.id GotUnfollow
                    )

                _ ->
                    ( model, Cmd.none )

        GotUnfollow (Ok ()) ->
            case model.user of
                Loaded user ->
                    ( { model
                        | user = Loaded { user | following = False }
                        , followState = Idle
                        , apiError = Nothing
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        GotUnfollow (Err err) ->
            ( { model
                | followState = Idle
                , apiError = Just err.message
              }
            , Cmd.none
            )

        ClickedEditProfile ->
            case model.user of
                Loaded user ->
                    ( { model
                        | profileState = EditingProfile { newName = user.name, newProfile = user.profile }
                        , apiError = Nothing
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        ClickedSaveEditedProfile ->
            case ( model.user, model.profileState ) of
                ( Loaded user, EditingProfile { newName, newProfile } ) ->
                    ( { model
                        | profileState = SentEditedProfile { newName = newName, newProfile = newProfile }
                        , apiError = Nothing
                      }
                    , Api.updateUser user.id { displayName = newName, profileText = newProfile } GotEditedProfile
                    )

                _ ->
                    ( model, Cmd.none )

        GotEditedProfile (Ok ()) ->
            case ( model.user, model.profileState ) of
                ( Loaded user, SentEditedProfile { newName, newProfile } ) ->
                    ( { model
                        | user = Loaded { user | name = newName, profile = newProfile }
                        , profileState = NotEditingProfile
                        , apiError = Nothing
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        GotEditedProfile (Err err) ->
            case model.profileState of
                SentEditedProfile { newName, newProfile } ->
                    ( { model
                        | profileState = EditingProfile { newName = newName, newProfile = newProfile }
                        , apiError = Just err.message
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        ClickedCancelEditingProfile ->
            ( { model
                | profileState = NotEditingProfile
                , apiError = Nothing
              }
            , Cmd.none
            )

        TypedName newName ->
            case model.profileState of
                EditingProfile new ->
                    ( { model | profileState = EditingProfile { new | newName = newName } }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        TypedProfile newProfile ->
            case model.profileState of
                EditingProfile new ->
                    ( { model | profileState = EditingProfile { new | newProfile = newProfile } }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        ClickedChangeAvatar ->
            ( model, File.Select.file [ "image/png", "image/jpg" ] SelectedNewAvatar )

        SelectedNewAvatar image ->
            ( { model
                | avatarState = EditingAvatar image
                , apiError = Nothing
              }
            , Task.perform SetNewAvatarUrl (File.toUrl image)
            )

        SetNewAvatarUrl url ->
            case model.avatarState of
                EditingAvatar image ->
                    ( { model
                        | avatarState = EditingAvatarUrl image url
                        , apiError = Nothing
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        ClickedSaveAvatar ->
            case ( model.user, model.avatarState ) of
                ( Loaded user, EditingAvatarUrl image url ) ->
                    ( { model
                        | avatarState = SentEditedAvatar image url
                        , apiError = Nothing
                      }
                    , Api.uploadAvatar user.id image GotNewAvatar
                    )

                _ ->
                    ( model, Cmd.none )

        GotNewAvatar (Ok avatar) ->
            case model.user of
                Loaded user ->
                    ( { model
                        | user = Loaded { user | avatar = avatar }
                        , avatarState = NotEditingAvatar
                        , apiError = Nothing
                      }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        GotNewAvatar (Err err) ->
            case model.avatarState of
                SentEditedAvatar image url ->
                    ( { model | avatarState = EditingAvatarUrl image url, apiError = Just err.message }
                    , Cmd.none
                    )

                _ ->
                    ( model, Cmd.none )

        Spin s ->
            case model.user of
                Loading spinner ->
                    ( { model | user = Loading (Spinner.update s spinner) }, Cmd.none )

                _ ->
                    ( model, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    case model.user of
        Loading _ ->
            Sub.map Spin Spinner.subscription

        _ ->
            Sub.none



-- VIEW


viewPostSmall : PostSmall -> Html Msg
viewPostSmall postSmall =
    div [ class "card", style "width" "10rem" ]
        [ div [ class "card-image" ]
            [ a [ href ("/post/" ++ String.fromInt postSmall.id) ]
                [ figure []
                    [ img [ src (Api.thumbSrc postSmall.thumbnail), style "object-fit" "cover", style "width" "10rem", style "height" "10rem", attribute "loading" "lazy" ] []
                    ]
                ]
            ]
        , div [ class "card-content p-1" ]
            [ div [ class "content" ]
                [ a [ href ("/post/" ++ String.fromInt postSmall.id) ]
                    [ p [ class "m-1", style "overflow" "hidden", style "white-space" "nowrap", style "text-overflow" "ellipsis" ]
                        [ text postSmall.title ]
                    ]
                ]
            ]
        ]


viewLoadedBody : Shared.Model -> UserView -> Model -> List (Html Msg)
viewLoadedBody shared user { profileState, avatarState, followState, apiError } =
    let
        { username, profile, editButton, saveButton, cancelButton } =
            case profileState of
                EditingProfile { newName, newProfile } ->
                    { username = input [ class "input", value newName, onInput TypedName ] []
                    , profile = textarea [ class "textarea", value newProfile, onInput TypedProfile ] []
                    , editButton = text ""
                    , saveButton = button [ class "button", onClick ClickedSaveEditedProfile ] [ text "Save" ]
                    , cancelButton = button [ class "button", onClick ClickedCancelEditingProfile ] [ text "Cancel" ]
                    }

                SentEditedProfile { newName, newProfile } ->
                    { username = input [ class "input", value newName, disabled True ] []
                    , profile = textarea [ class "textarea", value newProfile, disabled True ] []
                    , editButton = text ""
                    , saveButton = button [ class "button", onClick ClickedSaveEditedProfile, disabled True ] [ text "Save" ]
                    , cancelButton = button [ class "button", onClick ClickedCancelEditingProfile, disabled True ] [ text "Cancel" ]
                    }

                NotEditingProfile ->
                    { username = text user.name
                    , profile = text user.profile
                    , editButton =
                        case shared.user of
                            Just currentUser ->
                                if currentUser.id == user.id then
                                    button [ class "button", onClick ClickedEditProfile ] [ text "Edit profile" ]

                                else
                                    text ""

                            Nothing ->
                                text ""
                    , saveButton = text ""
                    , cancelButton = text ""
                    }

        followButton =
            case shared.user of
                Just currentUser ->
                    if currentUser.id == user.id then
                        -- user's own page
                        text ""

                    else
                        let
                            isDisabled =
                                followState /= Idle
                        in
                        if user.following then
                            -- following user
                            button [ class "button", onClick ClickedUnfollow, disabled isDisabled ] [ text "Unfollow" ]

                        else
                            -- not following user
                            button [ class "button", onClick ClickedFollow, disabled isDisabled ] [ text "Follow" ]

                Nothing ->
                    -- not logged in
                    text ""

        changeAvatarButton =
            case avatarState of
                NotEditingAvatar ->
                    case shared.user of
                        Just currentUser ->
                            if currentUser.id == user.id then
                                button [ class "button", onClick ClickedChangeAvatar ] [ text "Change avatar" ]

                            else
                                text ""

                        Nothing ->
                            text ""

                EditingAvatarUrl _ _ ->
                    button [ class "button", onClick ClickedSaveAvatar ] [ text "Save avatar" ]

                SentEditedAvatar _ _ ->
                    button [ class "button", disabled True ] [ text "Save avatar" ]

                _ ->
                    text ""

        posts =
            if List.isEmpty user.posts then
                text "No posts found"

            else
                div [ class "is-flex is-flex-direction-row is-flex-wrap-wrap" ] <|
                    List.map
                        (\post ->
                            div [ class "m-2" ]
                                [ viewPostSmall post ]
                        )
                        user.posts

        avatar =
            case avatarState of
                EditingAvatarUrl _ newAvatarUrl ->
                    img [ src newAvatarUrl, attribute "loading" "lazy" ] []

                _ ->
                    img [ src (Api.avatarSrc user.avatar), attribute "loading" "lazy" ] []
    in
    [ h1 [ class "block title" ] [ username ]
    , figure [ class "block image is-128x128" ]
        [ avatar ]
    , div [ class "block" ]
        [ followButton
        , changeAvatarButton
        , editButton
        , saveButton
        , cancelButton
        ]
    , div [ class "block", style "white-space" "break-spaces" ] [ profile ]
    , case apiError of
        Just e ->
            div [] [ text e ]

        Nothing ->
            text ""
    , posts
    ]


view : Shared.Model -> Model -> View Msg
view shared model =
    let
        contents =
            case model.user of
                Loading spinner ->
                    [ Components.Spinner.view spinner ]

                Errored err ->
                    [ Components.Error.view <| "Failed to get user: " ++ err ]

                Loaded user ->
                    viewLoadedBody shared user model
    in
    { title =
        case model.user of
            Loaded user ->
                user.name

            _ ->
                ""
    , body =
        [ Components.Section.view []
            contents
        ]
    }
