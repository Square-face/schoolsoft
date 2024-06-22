# The website

Schoolsoft's website is currently a mix of SSR and Client-Side-Rendering with
recent changes making the website use more CSR, specificaly the
schedule/calendar.

## Routes
The following is a description of the different api routes used by the website

### Login
Authenticate and get a session id for future api requests

Logging in is made using a POST request to
https://sms.schoolsoft.se/$school/jsp/Login.jsp with $school being
replaced with the actual [url name](#url-name) of a school.

The request must be made with form data holding the following keys and values:

```
action: login
usertype: 1
ssusername: $username
sspassword: $password
```

with $username and $password being replaced with the users actual username and
password.

If the login is successfull, the response will be a redirect (302 Found) to
`../jsp/student/right_student_startpage.jsp`. i.e the homepage for a logged in
user. If the login isn't successfull, the response will instead be a 200 OK.

Part of the redirect response is a set-cookie header that gives a JSESSIONID key
and value. This is how the api keeps track of the session, so as long as this is
part of the cookie header on future requests (and the session hasn't expired), interacting with the
api should just work.

## Dictionary
Some made up terms to describe parts of the api

### Url name
The url name is a url friendly version of the schools actual name to allow it to
be used in the context of a url without encoding.


