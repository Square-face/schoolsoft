POST https://sms.schoolsoft.se/{{school}}/jsp/Login.jsp
[FormParams]
action: login
usertype: 1
ssusername: {{username}}
sspassword: {{password}}

HTTP 302
[Captures]
sid: cookie "JSESSIONID"

GET https://sms.schoolsoft.se/minervagymnasium/rest-api/student/header/student
[Cookies]
JSESSIONID: {{sid}}
