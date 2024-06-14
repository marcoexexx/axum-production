# Auth flow

- [ Backend ] Login service
  - check user input
  - chcek user exits in database
  - check user verified
  - check user password
  - generate `access/refresh` tokens and set session user cache by user id
  - `access/refresh` store in cookie

- [ Backend ] Require user Middleware
  - check access token in request header with `Bearer` or in cookie
  - decode token and get session user
  - and check user in still there in real-database
  - set `request.extension.insert(user)`

- [ Frontend ] API interceptor response middleware
  - check no login _if no token, go auth/login route_
  - if could not refresh, go auth/login route
