POST https://sms.schoolsoft.se/{{school}}/rest/app/login
[FormParams]
identification: {{username}}
verification: {{password}}
logintype: 4
usertype: 1

HTTP 200
[Captures]
appkey: jsonpath "$['appKey']"

GET https://sms.schoolsoft.se/{{school}}/rest/app/token
appkey: {{appkey}}
# appos: android # aparently this is unecesary
# appversion: 2.3.14 # this to
deviceid: ''

HTTP 200
