import Html exposing (..)
import Html.Attributes exposing (..)
import Char
import Keyboard
import Http
import Json.Decode as Decode


type Model =
    Loading 
  | AllDone
  | HasList { head: String, tail: (List String) }
  | HasError String

type Msg = 
    GotList (Result Http.Error (List String))
  | GotRes (Result Http.Error ())
  | GotKeypress Int


main =
  Html.program {
    init = init,
    view = view,
    update = update,
    subscriptions = subscriptions }


init : (Model, Cmd Msg)
init =
  (Loading, getList)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    GotList (Ok list) ->
      next list
    GotList (Err err) ->
      (HasError (toString err), Cmd.none)
    GotRes (Ok ()) ->
      case model of
        HasList { tail } ->
          next tail
        _ ->
          (model, Cmd.none)
    GotRes (Err err) ->
      (HasError (toString err), Cmd.none)
    GotKeypress code ->
      case model of
        HasList { head } ->
          case (Char.fromCode code) of
            'y' ->
              (Loading, sendYes head)
            'n' ->
              (Loading, sendNo head)
            _ ->
              (model, Cmd.none)
        _ ->
          (model, Cmd.none)


view : Model -> Html Msg
view model =
  body [] [
    node "link" [rel "stylesheet", href "style.css"] [],
    case model of
      Loading ->
        h1 [] [text "loading..."]
      HasError err ->
        h1 [class "error"] [text err]
      HasList { head } ->
        img [src (head)] []
      AllDone ->
        h1 [] [text "all done."]
  ]


subscriptions model =
  Keyboard.presses GotKeypress


getList : Cmd Msg
getList =
  Http.send GotList (Http.get "http://192.168.1.141:3000/api/list" (Decode.list Decode.string))

sendYes: String -> Cmd Msg
sendYes id =
  Http.send GotRes (
    Http.post ("http://192.168.1.141:3000/api/yes/" ++ id) Http.emptyBody (Decode.succeed ()))

sendNo: String -> Cmd Msg
sendNo id =
  Http.send GotRes (
    Http.post ("http://192.168.1.141:3000/api/no/" ++ id) Http.emptyBody (Decode.succeed ()))

next : List String -> (Model, Cmd Msg)
next list =
  case (List.head list) of
    Just head ->
      (HasList { head = head, tail = (List.drop 1 list) }, Cmd.none)
    Nothing ->
      (AllDone, Cmd.none)
