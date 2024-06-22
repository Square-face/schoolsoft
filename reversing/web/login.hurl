POST https://sms.schoolsoft.se/{{school}}/jsp/Login.jsp
[FormParams]
action: login
usertype: 1
ssusername: {{username}}
sspassword: {{password}}

HTTP 302

[Asserts]
cookie "JSESSIONID" exists  # Session ID, use for further authentication
cookie "AWSALBCORS" exists  # AWS Load balancing
cookie "AWSALB" exists      # More AWS Load balancing
