#!/bin/bash

echo "installing TXT record"
echo "$DNS01_RECORD IN TXT \"$CERTBOT_VALIDATION\""


curl "http://api.dns-01.online/record" \
 -H "Content-Type: application/json" \
 -X POST \
 -d "{\"apikey\":\"$DNS01_APIKEY\",\"name\":\"$DNS01_RECORD\",\"token\":\"$CERTBOT_VALIDATION\",\"ttl\":86400}"

