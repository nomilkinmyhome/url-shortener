# Simple URL shortener - backend

### Backend endpoints

1. ```/?url=http://example.com/``` - generates and returns short-link.
2. ```/<short_link>``` - returns original URL for redirecting.

### Run

```cargo run```

### Run migrations

```diesel migration run```

#### rustc-nightly is required.
