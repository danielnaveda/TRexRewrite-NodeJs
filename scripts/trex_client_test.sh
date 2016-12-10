#!/bin/bash
printf "================= Testing TRex ================="; echo

for (( c=1; c<=1; c++ ))
do
# ./getconnection.sh

# source getconnection.sh

# ./subscribe.sh $CONN_ID
# ./subscribe.sh
source subscribe.sh

# ./getnotification.sh $CONN_ID

# ./publish.sh $CONN_ID
# ./publish.sh

# sleep 0.1

# ./getnotification.sh $CONN_ID

# ./getnotification.sh $CONN_ID

./unsubscribe.sh $CONN_ID
done
