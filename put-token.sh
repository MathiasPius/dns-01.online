#!/bin/bash

echo "installing TXT record for \"$DNS01_APIKEY\""
echo "$DNS01_RECORD IN TXT \"$CERTBOT_VALIDATION\""

curl "http://api.dns-01.online/record" \
 -H "Content-Type: application/json" \
 -X POST \
 -d "{\"apikey\":\"$DNS01_APIKEY\",\"name\":\"$DNS01_RECORD\",\"token\":\"$CERTBOT_VALIDATION\",\"ttl\":86400}"


# If we already have a TXT record on dns-01.online, LetsEncrypt might poll that
# before we get to update it, and we'll fail the challenge for using a statle token.
# By sleeping for a little while, we can be sure dns-01.online has had time to
# publish our new token
sleep 10

