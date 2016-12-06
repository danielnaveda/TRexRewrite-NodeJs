#!/bin/bash
printf "================= Testing TRex ================="; echo

# for (( c=1; c<=100; c++ ))
# do
# ./getconnection.sh

source getconnection.sh

./subscribe.sh $CONN_ID

./getnotification.sh $CONN_ID

./publish.sh $CONN_ID

./getnotification.sh $CONN_ID

./getnotification.sh $CONN_ID

# done
