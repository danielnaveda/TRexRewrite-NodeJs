# JSON sample data

This will be later formalized through the JSON schema

## Client -> Server

### Subscribe
```json
{
  "event_type": "42",
  "callback_url": "127.0.0.1/notification" (optional)
}
```

### Unsubscribe
```json
{
  "event_type": "42"
}
```

### Publish
```json
{
  "event" : <Event>
}
```

### Declare Event
```json
{
    "ty": "Event",
    "id": "1",
    "name": "temperature",
    "attributes": [
      {"name" : "area", "ty" : "Str"},
      /*...*/
      {"name" : "value", "ty" : "Int"}
    ]
}
```
<!-- TODO: finish the Rule definition -->
### Define Rule
```json
TODO
```

## Server->Client

### get connection
```json
{
  "conn_id": "12"
}
```

### get notification
```json
{
  "events" : {
      "Event",
      /*...*/
      "Event"
    }
}
```

## Common

### Event
```json
{
    "tuple": {
        "ty_id": "usize",
        "data": {
            "Int": "value",
            /*...*/
            "Str": "value"
        },
    },
    "time": "<time>"
}
```
