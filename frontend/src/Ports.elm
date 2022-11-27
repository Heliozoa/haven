port module Ports exposing (..)

import Bindings exposing (..)
import Json.Decode
import Json.Encode


port outgoing :
    { tag : String
    , content : Json.Decode.Value
    }
    -> Cmd msg


storeUser : User -> Cmd msg
storeUser user =
    outgoing
        { tag = "storeUser"
        , content = userEncoder user
        }


clearUser : Cmd msg
clearUser =
    outgoing
        { tag = "clearUser"
        , content = Json.Encode.null
        }
