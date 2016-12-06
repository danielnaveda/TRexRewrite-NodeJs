#!/bin/bash

printf "================= Testing TRex ================="; echo

# for (( c=1; c<=100; c++ ))
# do
./getconnection.sh

./subscribe.sh 123

./getnotification.sh

./publish.sh 333

./getnotification.sh

./getnotification.sh

# done
