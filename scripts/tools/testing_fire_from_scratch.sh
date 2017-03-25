#!/bin/bash
printf "================= Testing TRex ================="; echo

# ../operations/7_declare_event.sh "declare SMOKE(area: string) with id 0"
../operations/7_declare_event.sh "declare smoke(area: string) with id 0"

# ../operations/7_declare_event.sh "declare TEMPERATURE(area: string, value: int) with id 1"
../operations/7_declare_event.sh "declare temperature(area: string, value: int) with id 1"

# ../operations/7_declare_event.sh "declare FIRE(value:string, val:int) with id 2"
../operations/7_declare_event.sh "declare fire(area:string, temp:int) with id 2"

# ../operations/8_define_rule.sh "from 0[x = 0]() as SMK and last 1[y = 1](0 == x, 1 > 45) as TEMP within 5min from SMK emit 2(0 = x, 1 = y)"
../operations/8_define_rule.sh "from 0[x = 0]() as smk and last 1[y = 1](0 == x, 1 > 45) as temp within 5min from smk emit 2(0 = x, 1 = y)"
