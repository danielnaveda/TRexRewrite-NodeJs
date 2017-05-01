#!/bin/bash
printf "================= Testing TRex ================="; echo

../operations/7_declare_event.sh "declare smoke(area: string) with id 0"

../operations/7_declare_event.sh "declare temperature(area: string, value: int) with id 1"

../operations/7_declare_event.sh "declare fire(area:string, temp:int) with id 2"

# ../operations/8_define_rule.sh "from 0[x = 0]() as SMK and last 1[y = 1](0 == x, 1 > 45) as TEMP within 5min from SMK emit 2(0 = x, 1 = y)"
# ../operations/8_define_rule.sh "from 0[x = 0]() as smk and last 1[y = 1](0 == x, 1 > 45) as temp within 5min from smk emit 2(0 = x, 1 = y)"
../operations/8_define_rule.sh "from smoke[x = 0]() as smk and last temperature[y = 1](0 == x, 1 > 45) as temp within 5min from smk emit fire(0 = x, 1 = y)"

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
