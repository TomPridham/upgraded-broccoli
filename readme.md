Create a REST POST API in Rust that receives a CSV file as a POST body. It then parses this CSV and returns the same data as an array of JSON objects. The API must be able to handle multiple clients concurrently. Document the API. How would you test this?

Stretch goal - Store the data in a Postgres table so that it can be recalled via another API

Stretch goal - Create a SPA/page/component a user can use to upload a csv file to this API and get back the JSON


### API

#### Routes

* POST "/upload-csv"

    Accepts a single CSV file up to 1MB in size. The content of the CSV will be parsed into an array of JSON objects representing the parsed data. If the CSV is malformed, an error will be returned.

    Request: `body: FormData({file: <your_csv>})`

    Response: `Object[]`

#### Testing
##### Test cases
* basic case
* malformed csv
    * absolute trash
    * extra/missing columns compared to data
    * random holes in data
* large csv
* empty csv

The first failure point that I am not sure how best to test would be getting the path from the tempfile initially. If it is still buffered in memory, it won't have a path and `path()` returns `None`. The docs don't really specify when that might happen, so I think handling the `Result` and bailing on `Err` should be sufficient. The second failure point is if we are unable to open the `TempFile`. The most likely reason failure would be the file being cleaned up prematurely. The normal io failure modes also exist here. Handling the result should be sufficient for most cases. A potential issue that is called out in the docs is that there is the possibility of a collision if the `TempFile` is deleted and another `TempFile` is created at the same path. This would allow someone to access data that doesn't belong to them. Since we aren't specifying a path the possibility of collisions should be pretty low. If that is an unacceptable amount of risk, switching to a different handling of the uploads will be necessary.

##### Other considerations
Rocket should take care of handling for requests that don't match the expected schema, so testing for that shouldn't be necessary. It is also async and concurrent by default.

It isn't a terribly robust csv parsing strategy. I'm throwing out records that don't parse correctly with `flatten`. In a production grade project I would take more care to not throw out data or return an object like `{ok: Object[], errors:Error[]}` so that people can deal with the errors manually.

Since this is accepting arbitrary json, a basic schema would look something like this
```
id: SERIAL
filename: TEXT
parsed_json: JSON

```
I haven't had to deal with arbitrary data like this before, so I don't have any experience on what works or what doesn't. One thing that came to mind, but is almost certainly a long term bad idea is to store each file as its own table. That might be tempting to do because then you could have a perfect mapping, but that would quickly get out of hand and prolly cause some pretty bad performance problems.

The Rocket default file limit is 1MB. This is fine for something like this, but I could see that being much too small for production projects.

It just saves the file as `output.json`. There is an experimental file picker that only work in Chrome that could provide a better ux. It also requires that the site is served over HTTPS, so I didn't implement that. Alternatively, I could have saved the input filename and used that as the output filename, just changing the extension.
