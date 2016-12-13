#!/bin/bash
printf "================= Testing TRex ================="; echo

source subscribe.sh

source subscribe.sh

source subscribe.sh

curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"1\",\"data\": [\"area_1\", \"50\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;

curl -H "Content-Type: application/json" -X POST -d \
"{\"tuple\": {\"ty_id\": \"0\",\"data\": [\"area_1\"]},\"time\": \"2016-12-12T09:51:03.570254485Z\"}"\
 http://127.0.0.1:8888/events; echo; echo;




# ./getnotification.sh $CONN_ID

# ./publish.sh $CONN_ID
# ./publish.sh

# sleep 0.1

# ./getnotification.sh $CONN_ID

# ./getnotification.sh $CONN_ID

# ./unsubscribe.sh $CONN_ID
