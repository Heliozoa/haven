module Api exposing (..)

import Bindings exposing (..)
import File exposing (File)
import Http
import Json.Decode
import Json.Encode
import Url exposing (Protocol(..))



-- helpers


apiUrl : String
apiUrl =
    "http://127.0.0.1:3000"


type alias ApiResult ty =
    Result Error ty


type alias Request ty msg =
    (ApiResult ty -> msg) -> Cmd msg


get : String -> Json.Decode.Decoder ty -> Request ty msg
get url decoder expect =
    Http.riskyRequest
        { method = "GET"
        , headers = []
        , url = apiUrl ++ url
        , body = Http.emptyBody
        , expect = expectJson expect decoder
        , timeout = Nothing
        , tracker = Nothing
        }


post : { url : String, body : Http.Body, expect : Http.Expect msg } -> Cmd msg
post { url, body, expect } =
    Http.riskyRequest
        { method = "POST"
        , headers = []
        , url = apiUrl ++ url
        , body = body
        , expect = expect
        , timeout = Nothing
        , tracker = Nothing
        }


patch : { url : String, body : Http.Body, expect : Http.Expect msg } -> Cmd msg
patch { url, body, expect } =
    Http.riskyRequest
        { method = "PATCH"
        , headers = []
        , url = apiUrl ++ url
        , body = body
        , expect = expect
        , timeout = Nothing
        , tracker = Nothing
        }


put : { url : String, body : Http.Body, expect : Http.Expect msg } -> Cmd msg
put { url, body, expect } =
    Http.riskyRequest
        { method = "PUT"
        , headers = []
        , url = apiUrl ++ url
        , body = body
        , expect = expect
        , timeout = Nothing
        , tracker = Nothing
        }


delete : { url : String, body : Http.Body, expect : Http.Expect msg } -> Cmd msg
delete { url, body, expect } =
    Http.riskyRequest
        { method = "DELETE"
        , headers = []
        , url = apiUrl ++ url
        , body = body
        , expect = expect
        , timeout = Nothing
        , tracker = Nothing
        }


toApiResult : (String -> Result Error val) -> Http.Response String -> ApiResult val
toApiResult handler response =
    case response of
        Http.BadUrl_ url ->
            Err (Error ("Bad url: " ++ url))

        Http.Timeout_ ->
            Err (Error "Timed out")

        Http.NetworkError_ ->
            Err (Error "Network error")

        Http.BadStatus_ code body ->
            case Json.Decode.decodeString errorDecoder body of
                Ok err ->
                    Err (Error err.message)

                Err _ ->
                    Err (Error code.statusText)

        Http.GoodStatus_ _ body ->
            handler body


expectJson : (ApiResult a -> msg) -> Json.Decode.Decoder a -> Http.Expect msg
expectJson toMsg decoder =
    Http.expectStringResponse toMsg <|
        toApiResult
            (\body ->
                case Json.Decode.decodeString decoder body of
                    Ok value ->
                        Ok value

                    Err err ->
                        Err (Error (Json.Decode.errorToString err))
            )


expectWhatever : (ApiResult () -> msg) -> Http.Expect msg
expectWhatever toMsg =
    Http.expectStringResponse toMsg <|
        toApiResult
            (\_ ->
                Ok ()
            )


expectString : (ApiResult String -> msg) -> Http.Expect msg
expectString toMsg =
    Http.expectStringResponse toMsg <|
        toApiResult
            (\s ->
                Ok s
            )



-- static files


assetSrc : String -> String
assetSrc filename =
    apiUrl ++ "/static/asset/" ++ filename


avatarSrc : String -> String
avatarSrc filename =
    apiUrl ++ "/static/avatar/" ++ filename


imageSrc : String -> String
imageSrc filename =
    apiUrl ++ "/static/image/" ++ filename


thumbSrc : String -> String
thumbSrc filename =
    apiUrl ++ "/static/thumbnail/" ++ filename



-- api/auth


login : Login -> Request User msg
login credentials toMsg =
    post
        { url = "/api/auth/login"
        , body = Http.jsonBody (loginEncoder credentials)
        , expect = expectJson toMsg userDecoder
        }


logout : Request () msg
logout toMsg =
    post
        { url = "/api/auth/logout"
        , body = Http.emptyBody
        , expect = expectWhatever toMsg
        }


register : Registration -> Request () msg
register registration toMsg =
    post
        { url = "/api/auth/register"
        , body = Http.jsonBody (registrationEncoder registration)
        , expect = expectWhatever toMsg
        }



-- api/comics
-- api/lists
-- api/posts


getPosts : Request (List PostSmall) msg
getPosts =
    get
        "/api/posts"
        (Json.Decode.list postSmallDecoder)


getPost : String -> Request PostFull msg
getPost id =
    get ("/api/posts/" ++ id)
        postFullDecoder


type alias NewPost =
    { metadata : PostMetadata
    , images : List File
    }


uploadPost : NewPost -> Request String msg
uploadPost upload toMsg =
    post
        { url = "/api/posts"
        , body =
            Http.multipartBody
                (Http.stringPart "metadata" (Json.Encode.encode 0 (postMetadataEncoder upload.metadata))
                    :: List.map (\f -> Http.filePart "image" f) upload.images
                )
        , expect = expectString toMsg
        }


postComment : Int -> NewComment -> Request (List Comment) msg
postComment postId comment toMsg =
    post
        { url = "/api/posts/" ++ String.fromInt postId ++ "/comment"
        , body = Http.jsonBody (newCommentEncoder comment)
        , expect = expectJson toMsg (Json.Decode.list commentDecoder)
        }


findPosts : List String -> Request (List PostSmall) msg
findPosts tags =
    get ("/api/posts/search?" ++ List.foldr (++) "" (List.intersperse "," tags))
        (Json.Decode.list postSmallDecoder)


editComment : Int -> Int -> NewComment -> Request (List Comment) msg
editComment postId commentId comment toMsg =
    put
        { url = "/api/posts/" ++ String.fromInt postId ++ "/comment/" ++ String.fromInt commentId
        , body = Http.jsonBody (newCommentEncoder comment)
        , expect = expectJson toMsg (Json.Decode.list commentDecoder)
        }


deleteComment : Int -> Int -> Request (List Comment) msg
deleteComment postId commentId toMsg =
    delete
        { url = "/api/posts/" ++ String.fromInt postId ++ "/comment/" ++ String.fromInt commentId
        , body = Http.emptyBody
        , expect = expectJson toMsg (Json.Decode.list commentDecoder)
        }



-- api/tags


getTag : String -> Request Tag msg
getTag id =
    get ("/api/tags/" ++ id)
        tagDecoder



-- api/users


getUser : String -> Request UserView msg
getUser id =
    get ("/api/users/" ++ id)
        userViewDecoder


follow : Int -> Request () msg
follow userId toMsg =
    post
        { url = "/api/users/" ++ String.fromInt userId ++ "/follow"
        , body = Http.emptyBody
        , expect = expectWhatever toMsg
        }


unfollow : Int -> Request () msg
unfollow userId toMsg =
    post
        { url = "/api/users/" ++ String.fromInt userId ++ "/unfollow"
        , body = Http.emptyBody
        , expect = expectWhatever toMsg
        }


updateUser : Int -> UserUpdate -> Request () msg
updateUser userId userUpdate toMsg =
    patch
        { url = "/api/users/" ++ String.fromInt userId
        , body = Http.jsonBody (userUpdateEncoder userUpdate)
        , expect = expectWhatever toMsg
        }


uploadAvatar : Int -> File -> (Result Error String -> msg) -> Cmd msg
uploadAvatar userId avatar toMsg =
    post
        { url = "/api/users/" ++ String.fromInt userId ++ "/avatar"
        , body = Http.multipartBody [ Http.filePart "avatar" avatar ]
        , expect = expectString toMsg
        }
