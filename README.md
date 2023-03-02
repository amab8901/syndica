# Instructions

For Bash terminal:
1. `git clone https://github.com/amab8901/syndica`
2. `cd syndica`
3. `cargo run`

For Insomnia (API client):

4. POST request:
- Address: `127.0.0.1:3000/data`
- JSON body: 
```
{
	"val": 32383, 
	"id": "hej"
}
```
5. GET request:
- Address: `127.0.0.1:3000/data/:hej`
- (no body)
6. go back to step 4 (and then 5) with different `val`:s and different `id`:s. Note that 32383 is the maximum value allowed. Make sure that your choice of `id` is synchronized with the suffix of the address in the GET request in step 5. 

# Scenarios:

* If you run GET when the cache is empty (i.e. when you never used the server before), server will give 500 Internal Server Error and put 0 in the cache. If you run GET request the 2nd time, then it will return 0 from the cache.

* If your first request in a session is GET, then you will get non-cached value (regardless of your previous usage of POST request). 

* If your GET request is less than 30 seconds after a previous GET request, you will get cached value (regardless of your previous usage of POST request).

* If your GET request is more than 30 seconds after a previous GET request, you will get non-cached value (regardless of your previous usage of POST request).
