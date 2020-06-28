<h1>rust-backend-template</h1>

<b>Frameworks used:</b><br>

- Rocket - For managing the REST API
- Diesel - ORM that supports psql, mysql etc.
- slog - For logging

<br>

<strong> Steps to use: </strong>
<br><br>

- First install diesel_cli by using: `cargo install diesel`
- Initialize your new rust project with the template by using: <br>
`git clone https://github.com/smoke-trees/rust-backend-template`

- Create a `.env` file in the root directory of your project and set the `DATABASE_URL` variable

- Run `diesel setup` to create the database.

- Run `diesel migration run` to create all the necessary tables. Note that the table schemas
are written under migrations.

- If you want to create a new migration/table schema, run <br>`diesel migration create YOUR_MIGRATION_NAME`
<br>
<br>

<strong> Notes: </strong>
<br><br>

- Running migrations manually is only needed during development. The migrations are embedded in the binary and are executed at runtime during production.


<strong>Related links: </strong>

- Rocket: https://rocket.rs/
- Diesel: http://diesel.rs/


