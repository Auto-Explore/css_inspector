# css/css-conditional/container-queries/container-type-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-type-invalidation.html"
}
```

## style[0]

```css

  div {
    color: black;
  }
  #outer {
    width: 300px;
  }

  #intermediate {
    width: 250px;
  }

  #inner {
    width: 200px;
  }

  .container {
    container-type: inline-size;
  }

  @container ((max-width: 200px) or (min-width: 300px)) {
    #child { color: green; }
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
