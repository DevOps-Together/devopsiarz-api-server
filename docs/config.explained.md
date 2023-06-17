```json
{
    // Address for server to listen on
    "address":"0.0.0.0",
    // Port for server to listen on
    "port": 65535,
    // List of endpoints of server.
    "endpoints": [
        {
            // Path of endpoint. Paths cannot repeat.
            "path": "/wiadomosc",
            // Endpoint variant. Available variants are: Fifo, Filo, Constant, RepeatLast.
            "variant": "Fifo",
            // Enable or disable http basic authentication.
            "authentication_enabled": false,
            // Username. Must be set if authentication_enabled is set to true.
            "username": "",
            // Username. Must be set if authentication_enabled is set to true.
            "password": "",
            // Must be set if variant is set to CONST. Value will be sent as response.
            "const_response": ""
        }
    ]
}
```