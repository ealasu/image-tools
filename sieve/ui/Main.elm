import Html exposing (..)
import Html.Attributes exposing (..)
import Char
import Task
import Keyboard
import Http
import Json.Decode as Decode

main =
  Html.program {
    init = init,
    view = view,
    update = update,
    subscriptions = subscriptions }


type Model =
    Loading 
  | HasList { head: String, tail: (List String) }
  | HasListLoading Model
  | HasError String
  | AllDone

type Msg = 
    GotList (Result Http.Error (List String))
  | GotRes (Result Http.Error ())
  | GotKeypress Int



init : (Model, Cmd Msg)
init =
  (Loading, getList)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    GotList (Ok list) ->
      (next list, Cmd.none)
    GotList (Err err) ->
      (HasError ("GotList " ++ (toString err)), Cmd.none)
    GotRes (Ok ()) ->
      case model of
        HasListLoading nextModel ->
          (nextModel, Cmd.none)
        _ ->
          (model, Cmd.none)
    GotRes (Err err) ->
      (HasError ("GotRes " ++ (toString err)), Cmd.none)
    GotKeypress code ->
      case model of
        HasList { head, tail } ->
          case (Char.fromCode code) of
            'y' ->
              (HasListLoading (next tail), sendYes head)
            'n' ->
              (HasListLoading (next tail), sendNo head)
            _ ->
              (model, Cmd.none)
        _ ->
          (model, Cmd.none)


view : Model -> Html Msg
view model =
  body [] [
    node "link" [rel "stylesheet", href "style.css"] [],
    div [class "container"] [
      case model of
        Loading ->
          div [class "info"] [text "loading..."]
        HasError err ->
          div [class "info error"] [text err]
        HasList { head, tail } ->
          div [] [
            (case (List.head tail) of
              Just next ->
                --node "link" [rel "prefetch", href (baseUrl ++ "api/image/" ++ next)] []
                img [src (baseUrl ++ "api/image/" ++ next), style [("width", "1px"),("height", "1px")]] []
              Nothing ->
                div [] []),
            img [src (baseUrl ++ "api/image/" ++ head)] []
          ]
        HasListLoading _ ->
          div [class "info"] [text "loading image..."]
        AllDone ->
          div [class "info"] [text "all done."]
    ]
  ]


subscriptions model =
  Keyboard.presses GotKeypress


baseUrl = "http://192.168.1.141:3000/"

getList : Cmd Msg
getList =
  Http.send GotList (Http.get (baseUrl ++ "api/list") (Decode.list Decode.string))

sendNo: String -> Cmd Msg
sendNo id =
  post (baseUrl ++ "api/move/" ++ id ++ "/bad")
    |> Http.send GotRes

sendYes: String -> Cmd Msg
sendYes id =
  post (baseUrl ++ "api/move/" ++ id ++ "/good")
    |> Http.send GotRes

next : List String -> Model
next list =
  case (List.head list) of
    Just head ->
      HasList { head = head, tail = (List.drop 1 list) }
    Nothing ->
      AllDone

post : String -> Http.Request ()
post url =
  Http.request
    { method = "POST"
    , headers = []
    , url = url
    , body = Http.emptyBody
    , expect = Http.expectStringResponse (\_ -> Ok ())
    , timeout = Nothing
    , withCredentials = False
    }
