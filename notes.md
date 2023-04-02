Git app:

- one program for admin control of a gifs account
  - all gifs stored in this account
  - authority is the signer
  - `access_control` used on the `add_gif` method to only allow authority to call it
  - program can control users in second program below via PDA to `grant_access_to_gif`
    - signed by the admin

- second program for user's gif_list retrieval
  - each user inits to create an account
  - `get_gif_list` method lists the user's allowed gifs