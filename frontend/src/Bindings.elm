
-- generated by elm_rs


module Bindings exposing (..)

import Dict exposing (Dict)
import Http
import Json.Decode
import Json.Encode
import Url.Builder


resultEncoder : (e -> Json.Encode.Value) -> (t -> Json.Encode.Value) -> (Result e t -> Json.Encode.Value)
resultEncoder errEncoder okEncoder enum =
    case enum of
        Ok inner ->
            Json.Encode.object [ ( "Ok", okEncoder inner ) ]
        Err inner ->
            Json.Encode.object [ ( "Err", errEncoder inner ) ]


resultDecoder : Json.Decode.Decoder e -> Json.Decode.Decoder t -> Json.Decode.Decoder (Result e t)
resultDecoder errDecoder okDecoder =
    Json.Decode.oneOf
        [ Json.Decode.map Ok (Json.Decode.field "Ok" okDecoder)
        , Json.Decode.map Err (Json.Decode.field "Err" errDecoder)
        ]


type alias Login =
    { email : String
    , password : String
    }


loginEncoder : Login -> Json.Encode.Value
loginEncoder struct =
    Json.Encode.object
        [ ( "email", (Json.Encode.string) struct.email )
        , ( "password", (Json.Encode.string) struct.password )
        ]


type alias Registration =
    { name : String
    , email : String
    , password : String
    }


registrationEncoder : Registration -> Json.Encode.Value
registrationEncoder struct =
    Json.Encode.object
        [ ( "name", (Json.Encode.string) struct.name )
        , ( "email", (Json.Encode.string) struct.email )
        , ( "password", (Json.Encode.string) struct.password )
        ]


type alias NewComment =
    { text : String
    }


newCommentEncoder : NewComment -> Json.Encode.Value
newCommentEncoder struct =
    Json.Encode.object
        [ ( "text", (Json.Encode.string) struct.text )
        ]


type alias UserUpdate =
    { displayName : String
    , profileText : String
    }


userUpdateEncoder : UserUpdate -> Json.Encode.Value
userUpdateEncoder struct =
    Json.Encode.object
        [ ( "display_name", (Json.Encode.string) struct.displayName )
        , ( "profile_text", (Json.Encode.string) struct.profileText )
        ]


type alias PostMetadata =
    { title : String
    , post : String
    , tags : List (Int)
    , thumbnail : Thumbnail
    , nsfw : Bool
    }


postMetadataEncoder : PostMetadata -> Json.Encode.Value
postMetadataEncoder struct =
    Json.Encode.object
        [ ( "title", (Json.Encode.string) struct.title )
        , ( "post", (Json.Encode.string) struct.post )
        , ( "tags", (Json.Encode.list (Json.Encode.int)) struct.tags )
        , ( "thumbnail", (thumbnailEncoder) struct.thumbnail )
        , ( "nsfw", (Json.Encode.bool) struct.nsfw )
        ]


type alias Thumbnail =
    { idx : Int
    , x : Int
    , y : Int
    , size : Int
    }


thumbnailEncoder : Thumbnail -> Json.Encode.Value
thumbnailEncoder struct =
    Json.Encode.object
        [ ( "idx", (Json.Encode.int) struct.idx )
        , ( "x", (Json.Encode.int) struct.x )
        , ( "y", (Json.Encode.int) struct.y )
        , ( "size", (Json.Encode.int) struct.size )
        ]


type alias User =
    { id : Int
    , name : String
    }


userEncoder : User -> Json.Encode.Value
userEncoder struct =
    Json.Encode.object
        [ ( "id", (Json.Encode.int) struct.id )
        , ( "name", (Json.Encode.string) struct.name )
        ]


type alias PostSmall =
    { id : Int
    , thumbnail : String
    , title : String
    , userId : Int
    , username : String
    }


postSmallDecoder : Json.Decode.Decoder PostSmall
postSmallDecoder =
    Json.Decode.succeed PostSmall
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "thumbnail" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "title" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "user_id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "username" (Json.Decode.string)))


type alias PostFull =
    { id : Int
    , images : List (Image)
    , tags : List (TagEmbed)
    , title : String
    , text : String
    , userId : Int
    , username : String
    , avatar : String
    , comments : List (Comment)
    }


postFullDecoder : Json.Decode.Decoder PostFull
postFullDecoder =
    Json.Decode.succeed PostFull
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "images" (Json.Decode.list (imageDecoder))))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "tags" (Json.Decode.list (tagEmbedDecoder))))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "title" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "text" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "user_id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "username" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "avatar" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "comments" (Json.Decode.list (commentDecoder))))


type alias Image =
    { id : Int
    , filename : String
    , height : Int
    , width : Int
    }


imageDecoder : Json.Decode.Decoder Image
imageDecoder =
    Json.Decode.succeed Image
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "filename" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "height" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "width" (Json.Decode.int)))


type alias Comment =
    { id : Int
    , userId : Int
    , username : String
    , text : String
    , avatar : String
    }


commentDecoder : Json.Decode.Decoder Comment
commentDecoder =
    Json.Decode.succeed Comment
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "user_id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "username" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "text" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "avatar" (Json.Decode.string)))


userDecoder : Json.Decode.Decoder User
userDecoder =
    Json.Decode.succeed User
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "name" (Json.Decode.string)))


type alias Tag =
    { id : Int
    , alias : String
    , slug : String
    , differentiator : Maybe (String)
    , description : Maybe (String)
    , image : Maybe (Int)
    }


tagDecoder : Json.Decode.Decoder Tag
tagDecoder =
    Json.Decode.succeed Tag
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "alias" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "slug" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "differentiator" (Json.Decode.nullable (Json.Decode.string))))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "description" (Json.Decode.nullable (Json.Decode.string))))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "image" (Json.Decode.nullable (Json.Decode.int))))


type alias TagEmbed =
    { id : Int
    , aliasName : String
    , slug : String
    , differentiator : Maybe (String)
    , image : Maybe (Int)
    }


tagEmbedDecoder : Json.Decode.Decoder TagEmbed
tagEmbedDecoder =
    Json.Decode.succeed TagEmbed
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "alias_name" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "slug" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "differentiator" (Json.Decode.nullable (Json.Decode.string))))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "image" (Json.Decode.nullable (Json.Decode.int))))


type alias TagView =
    { tag : Tag
    , posts : List (PostSmall)
    }


tagViewDecoder : Json.Decode.Decoder TagView
tagViewDecoder =
    Json.Decode.succeed TagView
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "tag" (tagDecoder)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "posts" (Json.Decode.list (postSmallDecoder))))


type alias UserView =
    { id : Int
    , name : String
    , avatar : String
    , profile : String
    , created : String
    , following : Bool
    , posts : List (PostSmall)
    }


userViewDecoder : Json.Decode.Decoder UserView
userViewDecoder =
    Json.Decode.succeed UserView
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "id" (Json.Decode.int)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "name" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "avatar" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "profile" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "created" (Json.Decode.string)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "following" (Json.Decode.bool)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "posts" (Json.Decode.list (postSmallDecoder))))


type alias Error =
    { message : String
    }


errorDecoder : Json.Decode.Decoder Error
errorDecoder =
    Json.Decode.succeed Error
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "message" (Json.Decode.string)))


