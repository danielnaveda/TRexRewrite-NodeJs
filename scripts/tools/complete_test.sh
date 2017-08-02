#!/bin/bash
printf "================= Testing TRex ================="; echo

# ../operations/7_declare_event.sh "declare eventone(aint: int, afloat: float, abool: bool, astring: string) with id 0"
../operations/7_declare_event.sh "declare eventone(aint: int, astring: string, afloat: float, abool: bool) with id 0"

# ../operations/7_declare_event.sh "declare eventtwo(bint: int, bfloat: float, bbool: bool, bstring: string) with id 1"
../operations/7_declare_event.sh "declare eventtwo(bint: int, bstring: string, bfloat: float, bbool: bool) with id 1"

# ../operations/7_declare_event.sh "declare eventthree(cint: int, cfloat: float, cbool: bool, cstring: string) with id 2"
../operations/7_declare_event.sh "declare eventthree(cint: int, cstring: string, cfloat: float, cbool: bool) with id 2"

# ../operations/8_define_rule.sh "from eventone[x = aint, x2 = afloat, x3 = abool, x4 = astring]() as eone and last eventtwo[y = aint]() as etwo within 5min from eone emit eventthree(aint = x, afloat = x2, abool = x3, astring = x4)"
# ../operations/8_define_rule.sh "from eventone[xone = aint, xtwo = afloat, xthree = abool, xfour = astring]() as eone and last eventtwo[y = bint]() as etwo within 5min from eone emit eventthree(cint = xone, cfloat = xtwo, cbool = xthree, cstring = xfour)"
../operations/8_define_rule.sh "from eventone[xone = aint, xfour = astring, xtwo = afloat, xthree = abool]() as eone and last eventtwo[y = bint]() as etwo within 5min from eone emit eventthree(cint = xone, cstring = xfour, cfloat = xtwo, cbool = xthree)"
exit

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
