# Requirements

- Internal api will call external api with access token.
- If external api will respond with status code 401.
- Hold all the api calls for that particular external service.
- A single thread will be spawned which will get the new access token based on the refresh token.
- Then make api call to the external service with the new access token.

- Ran : `wrk -t20 -c20 -d10s http://localhost:3050/`
    ```text

    Running 10s test @ http://localhost:3050/
    20 threads and 20 connections
    Thread Stats   Avg      Stdev     Max   +/- Stdev
        Latency    18.74ms    2.66ms  49.37ms   89.66%
        Req/Sec    53.41      5.87    60.00     59.20%
    10719 requests in 10.06s, 1.24MB read
    Requests/sec:   1065.88
    Transfer/sec:    125.95KB
    ```