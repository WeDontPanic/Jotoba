**Suggestion endpoint**
----
  Fetch word suggestions for a search query

* **URL**

  /api/suggestion

* **Method:**

  `POST`
  
* **Data Params**

   **Required:**
 
   `input=[string]` // Represents the input query to generate the 

   **Optional:**
 
   `lang=[string]` // The language set by the user. Default is english. Words will always match english<br>
                   // en-US, de-DE, es-ES ,fr-FR ,nl-NL ,sv-SE ,ru ,hu ,sl-SI

   **Exmple:**
   ```
   {
     "input": "まち",
     "lang": "de-DE",
   }
   ```

* **Success Response:**

  * **Code:** 200 <br />
    **Content:** `{ "suggestions":[ {"primary":"まち","secondary":"町"} ], "suggestion_type": "default" }`
 
* **Error Response:**

  * **Code:** 400 BAD_REQUEST <br />
    **Content:** `{ "code" : 400, "error": "BadRequest", "message": "Bad request" }`

  OR

  * **Code:** 500 INTERNAL <br />
    **Content:** `{ "code" : 500, "error": "InternalError", "message": "Internal server error" }`

  OR

  * **Code:** 408 TIMEOUT <br />
    **Content:** `{ "code" : 408, "error": "Timeout", "message": "Timeout exceeded" }`

* **Sample Call:**

  ```
  curl -XPOST https://jotoba.de/api/suggestion \
       -H "Content-Type: application/json" \
       --data '{"input": "まち"}'
  ```

* **Notes:**

  Each server has a set timeout for how long search suggestion queries are alowed to run until they time out. This may vary from instance to instance. Default is set to 100ms
