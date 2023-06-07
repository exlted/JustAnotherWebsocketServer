# JustAnotherWebsocketServer

This is just another websocket server, a webserver that provides a super basic interface for serving multiple sites that can communicate with each other over a shared websocket connection.

Customizations are placed in a location defined in the docker-compose.yml and served based on the name of the folder using a url parameter like this: "?customization=example"