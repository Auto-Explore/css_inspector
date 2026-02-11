# css/css-conditional/container-queries/idlharness.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/idlharness.html"
}
```

## style[0]

```css

  @container cont (width = 100px) {
    @container (inline-size > 200em) {
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
