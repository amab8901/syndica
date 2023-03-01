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
	"val": 12345, 
	"id": "hej"
}
```
5. GET request:
- Address: `127.0.0.1:3000/data/:hej`
- (no body)
6. go back to step 4 (and then 5) with different values and different `id`:s. Make sure that your choice of `id` is synchronized with the suffix of the address in the GET request in step 6. 