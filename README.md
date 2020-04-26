# Waxy

A prototype HTTP proxy that uses a WASM module to perform header-based request authorization.

## How it works

The main `waxy` program starts up configured with the path to a WASM module. This module is
expected to implement two functions: `admit_ptr`, which returns a pointer to memory where a value
should be written; and `admit`, which takes a pointer to a string (and its lenght) and returns a
1 if this value should be admitted and 0 if it should not.

The `waxy` proxy reads a single, arbitrary header from the request

## Example: Admit `password`

The `waxy-admit-password` crate implements a WASM module that only admits requests when the header value is `password`:

```sh
:;  (cd waxy-admit-password && cargo build --release)
:; cargo run target/wasm32-unknown-unknown/release/waxy_admit_password.wasm
```

This denies requests by default:

```sh
:; curl -x http://localhost:8000 httpbin.org/status/200
*   Trying ::1:8000...
* TCP_NODELAY set
* connect to ::1 port 8000 failed: Connection refused
*   Trying 127.0.0.1:8000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 8000 (#0)
> GET http://httpbin.org/status/200 HTTP/1.1
> Host: httpbin.org
> User-Agent: curl/7.65.3
> Accept: */*
> Proxy-Connection: Keep-Alive
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 403 Forbidden
< content-length: 0
< date: Sun, 26 Apr 2020 23:38:48 GMT
<
* Connection #0 to host localhost left intact
```

But a magic header value can be set to permit requests to proceed:

```sh
:; curl -x http://localhost:8000 -H 'waxy-admit: password' -v httpbin.org/status/200
*   Trying ::1:8000...
* TCP_NODELAY set
* connect to ::1 port 8000 failed: Connection refused
*   Trying 127.0.0.1:8000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 8000 (#0)
> GET http://httpbin.org/status/200 HTTP/1.1
> Host: httpbin.org
> User-Agent: curl/7.65.3
> Accept: */*
> Proxy-Connection: Keep-Alive
> waxy-admit: password
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< date: Sun, 26 Apr 2020 23:11:33 GMT
< content-type: text/html; charset=utf-8
< connection: keep-alive
< server: gunicorn/19.9.0
< access-control-allow-origin: *
< access-control-allow-credentials: true
< content-length: 0
<
* Connection #0 to host localhost left intact
```

## Example: Deny `rhubab`

```sh
:; (cd waxy-deny-rhubarb && cargo build --release)
:; cargo run target/wasm32-unknown-unknown/release/waxy_deny_rhubarb.wasm
```

This module allows arbitrary requests through:

```sh
:; curl -x http://localhost:8000 -v httpbin.org/status/200
*   Trying ::1:8000...
* TCP_NODELAY set
* connect to ::1 port 8000 failed: Connection refused
*   Trying 127.0.0.1:8000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 8000 (#0)
> GET http://httpbin.org/status/200 HTTP/1.1
> Host: httpbin.org
> User-Agent: curl/7.65.3
> Accept: */*
> Proxy-Connection: Keep-Alive
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< date: Sun, 26 Apr 2020 23:46:00 GMT
< content-type: text/html; charset=utf-8
< connection: keep-alive
< server: gunicorn/19.9.0
< access-control-allow-origin: *
< access-control-allow-credentials: true
< content-length: 0
<
* Connection #0 to host localhost left intact
```

Unless the header value is `rhubarb`:

```sh
:; curl -x http://localhost:8000 -H 'waxy-admit: rhubarb' -v httpbin.org/status/200
*   Trying ::1:8000...
* TCP_NODELAY set
* connect to ::1 port 8000 failed: Connection refused
*   Trying 127.0.0.1:8000...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 8000 (#0)
> GET http://httpbin.org/status/200 HTTP/1.1
> Host: httpbin.org
> User-Agent: curl/7.65.3
> Accept: */*
> Proxy-Connection: Keep-Alive
> waxy-admit: rhubarb
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 403 Forbidden
< content-length: 0
< date: Sun, 26 Apr 2020 23:40:48 GMT
<
* Connection #0 to host localhost left intact
```
