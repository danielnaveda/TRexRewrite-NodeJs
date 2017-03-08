#!/bin/bash
printf "================= Testing TRex ================="; echo

for (( c=1; c<=3; c++ ))
do
  # Get connection ID
  OUTPUT=$(curl --fail --silent --show-error http://127.0.0.1:8888/connections)
  # CONN_ID=$(echo $OUTPUT | jq -r '.value')
  CONN_ID[c]=$(echo $OUTPUT | jq -r '.value')

  # Subscribe
  # The subscription ID is not taken into account because the engine does not distinguish event types
  # ../operations/2_subscribe.sh ${CONN_ID[c]} 0

  OUTPUT=$(curl --fail --silent --show-error -H "Content-Type: application/json" -X POST http://127.0.0.1:8888/subscriptions/${CONN_ID[c]}/0)
  SUBS_ID[c]=$(echo $OUTPUT | jq -r '.value')
  # echo "$OUTPUT"; echo

  sleep 0.5
done

# Publish event
# Temperature
curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"1\",\"data\": [\"area_1\", \"45\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;

sleep 1

 # Temperature
 curl -H "Content-Type: application/json" -X POST -d \
 "{\"tuple\": {\"ty_id\": \"1\",\"data\": [\"area_1\", \"50\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
  http://127.0.0.1:8888/events; echo; echo;

sleep 1

# Smoke
curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"0\",\"data\": [\"area_1\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;

sleep 1

for (( c=1; c<=3; c++ ))
do
 # Get Notifications
 ../operations/4_getnotification.sh ${CONN_ID[c]}
 ../operations/4_getnotification.sh ${CONN_ID[c]}
 ../operations/4_getnotification.sh ${CONN_ID[c]}
 ../operations/4_getnotification.sh ${CONN_ID[c]}
 sleep 0.5
done


for (( c=1; c<=3; c++ ))
do
  # Unsubscribe
  ../operations/3_unsubscribe.sh ${CONN_ID[c]} ${SUBS_ID[c]}

 sleep 0.5
done

# Publish event
# Temperature
curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"1\",\"data\": [\"area_1\", \"45\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;
