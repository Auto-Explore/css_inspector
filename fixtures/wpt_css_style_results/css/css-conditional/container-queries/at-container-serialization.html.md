# css/css-conditional/container-queries/at-container-serialization.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/at-container-serialization.html"
}
```

## style[0]

```css

  @container (width=100px) {
    @container \!-name (inline-size > 200px    ) {
      #id { color: lime }
    }
    #id { color: green }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
