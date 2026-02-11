# css/css-conditional/container-queries/backdrop-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/backdrop-invalidation.html"
}
```

## style[0]

```css

  :root {
    color: black;
  }

  #container {
    container-type: size;
    width: 200px;
    height: 40px;
  }

  ::backdrop {
    background-color: black;
  }

  @container (min-width: 300px) {
    ::backdrop {
      background-color: green;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
