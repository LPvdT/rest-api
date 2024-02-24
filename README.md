# REST API

A very simple REST API written in Rust, powered by the `actix-web` crate.

## Compiling

Compile the application for your OS/architecture, using `cargo build --release`.

## Endpoints

### `GET` *health_check*

+ `http://localhost:8080`

### `GET` *get_entries*

+ `http://localhost:8080/todolist/entries`

### `POST` *create_entry*

+ `http://localhost:8080/todolist/entries`

```json
{
  "title": "Title",
  "date": 1665230864337
}
```

### `PUT` *update_entry*

+ `http://localhost:8080/todolist/entries/{id}`

```json
{
  "title": "New Title"
}
```

### `DELETE` *delete_entry*

+ `http://localhost:8080/todolist/entries/{id}`

## Testing the endpoints

+ Import `thunder-collection_todolist_api.json` from `.thunderclient` using the [VSCode Thunder Client extension](https://marketplace.visualstudio.com/items?itemName=rangav.vscode-thunder-client) to load the requests/tests.
+ Make requests to the endpoints using the extension.

Alternatively, you can also use a tool like `curl`.
