#!/bin/bash
printf "================= Testing TRex ================="; echo

# Get connection ID
OUTPUT=$(curl --fail --silent --show-error http://127.0.0.1:8888/connections)
CONN_ID=$(echo $OUTPUT | jq -r '.value')

# The subscription ID is not taken into account because the engine does not distinguish event types
../operations/2_subscribe.sh $CONN_ID 0

# Temperature
curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"1\",\"data\": [\"area_1\", \"50\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;

# Smoke
curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"0\",\"data\": [\"area_1\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;
