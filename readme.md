# Requirements

- Internal api will call external api with access token.
- If external api will respond with status code 401.
- Hold all the api calls for that particular external service.
- A single thread will be spawned which will get the new access token based on the refresh token.
- Then make api call to the external service with the new access token.