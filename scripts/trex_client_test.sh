#!/bin/bash

#
echo "--------------- Testing TRex ---------------"; echo


#Get Connection ID
echo "Get Connection ID:"
curl http://127.0.0.1:8888/connections; echo; echo

#Subscribe
echo "Subscribe:"
curl -H "Content-Type: application/json" -X POST http://127.0.0.1:8888/subscriptions/123456; echo; echo

#Get Notification
echo "Get Notification:"
curl http://127.0.0.1:8888/events/123456; echo; echo

#Publish
echo "Publish:"
curl -H "Content-Type: application/json" -X POST -d '{"value": 18}' http://127.0.0.1:8888/events/123456; echo; echo

#Get Notification
echo "Get Notification:"
curl http://127.0.0.1:8888/events/123456; echo; echo

#Get Notification
echo "Get Notification:"
curl http://127.0.0.1:8888/events/123456; echo; echo
