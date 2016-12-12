# JSON sample data

This will be later formalized through the JSON schema

## Client -> Server

### Subscribe
```json
{
  "event_type": "42"
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
  "event" : "Event"
}
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
