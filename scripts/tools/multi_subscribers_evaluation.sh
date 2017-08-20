#!/bin/bash
printf "================= Testing TRex ================="; echo

N_SUBSCRIBERS=500

for (( c=0; c<$N_SUBSCRIBERS; c++ ))
do
  # Get connection ID
  OUTPUT=$(curl --fail --silent --show-error http://127.0.0.1:8888/connections)
  CONN_ID[c]=$(echo $OUTPUT | jq -r '.value')

  # Subscribe
  ../operations/2_subscribe.sh ${CONN_ID[c]} 0
done

# Publish event
# Temperature
curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"1\",\"data\": [\"area_1\", \"45\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;

 for (( c=0; c<$N_SUBSCRIBERS; c++ ))
 do
  # Get Notifications
  ../operations/4_getnotification.sh ${CONN_ID[c]}
 done
