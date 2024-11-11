# Simple WebSocket Chat Server

Wanted to do it asynchronous, but considered to stick to the MIO philosophy and implemented everything on raw threads.
Program is rather synchronous.

Disclaimer: Code is **NOT** clean. Code is **NOT** maintainable. I just wanted to test `ws` crate

## Usage

Start by `cargo run`

Listens to `0.0.0.0:8080`

Last part of the URL should be username

`127.0.0.1:8080/whatever/John` will connect you with username "John"

You can send messages to a chat by sending following JSON

```json
{
  "content": "Text"
}
```

There are 4 `kind`s of messages

### `Hello`

It is the first message that every client receives on connect

```json
{
  "kind": "Hello",
  "clients": [
    "Kristy",
    "Mike"
  ],
  "history": [
    {
      "kind": "Connected",
      "client": "Mike",
      "at": "2024-11-11T01:13:26.283179200Z"
    },
    {
      "kind": "Connected",
      "client": "Kristy",
      "at": "2024-11-11T01:13:30.343416100Z"
    },
    {
      "kind": "Message",
      "from": "Kristy",
      "content": "Hi Mike!",
      "at": "2024-11-11T01:13:30.938304500Z"
    },
    {
      "kind": "Message",
      "from": "Mike",
      "content": "Wassup Kristy",
      "at": "2024-11-11T01:13:47.522515900Z"
    },
    {
      "kind": "Connected",
      "client": "John",
      "at": "2024-11-11T01:13:51.933189800Z"
    }
  ]
}
```

### `Connected`

User had connected to a chat

```json
{
  "kind": "Connected",
  "client": "John",
  "at": "2024-11-11T01:13:51.933189800Z"
}
```

### `Disconnected`

User had disconnected from a chat

```json
{
  "kind": "Disconnected",
  "client": "Mike",
  "at": "2024-11-11T01:14:51.509437500Z"
}
```

### `Message`

Message sent by other user

```json
{
  "kind": "Message",
  "from": "Mike",
  "content": "Wassup Kristy",
  "at": "2024-11-11T01:10:47.410114700Z"
}
```


