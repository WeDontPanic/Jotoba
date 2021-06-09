**Suggestion endpoint**
----
  Fetch word suggestions for a search query

* **URL**

  /api/suggestion

* **Method:**

  `POST`
  
*  **URL Params**

   **Required:**
 
   `input=[string]` // Represents the input query to generate the 

   **Optional:**
 
   `lang=[integer]` // The language set by the user. Default is english. Words will always match english

* **Data Params**

  ```
  {
    "input": "まち",
    "lang": 0,
  }
  ```

* **Success Response:**

  * **Code:** 200 <br />
    **Content:** `{ "suggestions":[ {"primary":"まち","secondary":"町"} ] }`
 
* **Error Response:**

  * **Code:** 400 BAD_REQUEST <br />
    **Content:** `{ "code" : 400, "error": "BadRequest", "message": "Bad request" }`

  OR

  * **Code:** 500 INTERNAL <br />
    **Content:** `{ "code" : 500, "error": "InternalError", "message": "Internal server error" }`

  OR

  * **Code:** 408 TIMEOUT <br />
    **Content:** `{ "code" : 408, "error": "Timeout", "message": "Query timed out" }`

* **Sample Call:**

  ```
  curl -XPOST \
          127.0.0.1:8080/api/suggestion\
          -H "Content-Type: application/json" \
          --data '{"input": "まち"}'
  ```

* **Notes:**

  Each server has a set timeout for how long search suggestion queries are alowed to run until they time out. This may vary from instance to instance. Default is set to 100ms
