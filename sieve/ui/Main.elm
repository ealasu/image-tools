import Html exposing (..)
import Html.Attributes exposing (..)
import Char
import Keyboard
import Http
import Json.Decode as Decode


type Model =
    Loading 
  --| AllDone
  | HasList (List String)
  --| HasList { head: String, tail: (List String) }
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
      (HasList list, Cmd.none)
    GotList (Err err) ->
      (HasError (toString err), Cmd.none)
    GotRes (Ok ()) ->
      case model of
        HasList list ->
          (HasList (List.drop 1 list), Cmd.none)
        _ ->
          (model, Cmd.none)
    GotRes (Err err) ->
      (HasError (toString err), Cmd.none)
    GotKeypress code ->
      case model of
        HasList list ->
          case (List.head list) of
            Just id ->
              case (Char.fromCode code) of
                'r' ->
                  (Loading, Cmd.none)
                'y' ->
                  (Loading, sendYes id)
                'n' ->
                  (Loading, sendNo id)
                _ ->
                  (model, Cmd.none)
            Nothing ->
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
      HasList list ->
        case (List.head list) of
          Just imageUrl ->
            img [src (imageUrl)] []
          Nothing ->
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
