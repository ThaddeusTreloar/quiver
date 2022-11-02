#!/bin/bash

openssl ecparam -name secp384r1 -genkey -noout -out ./run/private.pem
openssl ec -in ./run/private.pem -pubout -out ./run/public.pem