**Radical endpoint**
----
  Search kanji by the radicals used to build it.

* **URL**

  /api/kanji/by_radical

* **Data Params**

   **Required:**
 
   `radicals=[char[]]` // One or multiple radicals the kanji should be built with

   **Exmple:**
   
  ```
  { "radicals": [ "山", "一", "冂" , "干"] }
  ```

* **Success Response:**

  * **Code:** 200 <br />
    **Content:**
    ```
      {
        "kanji": {
          "8": [
            "岡"
          ],
          "14": [
            "綱"
          ]
        },
        "possible_radicals": [
          "小",
          "岡",
          "幺",
          "糸"
        ]
      }
    ```
 
* **Error Response:**

   * **Code:** 400 BAD_REQUEST <br />
    **Content:** `{ "code" : 400, "error": "BadRequest", "message": "Bad request" }`

  OR

  * **Code:** 500 INTERNAL <br />
    **Content:** `{ "code" : 500, "error": "InternalError", "message": "Internal server error" }`

* **Sample Call:**

  ```
  curl  -XPOST https://jotoba.de/api/kanji/by_radical \
        -H "Content-Type: application/json" \
        --data '{"radicals":[ "山", "一", "冂", "干" ]}'
  ```

* **Notes:**

  The limit of radicals which can be passed is `12`
