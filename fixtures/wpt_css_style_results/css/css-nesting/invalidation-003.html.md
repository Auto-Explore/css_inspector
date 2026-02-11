# css/css-nesting/invalidation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/invalidation-003.html"
}
```

## style[0]

```css

  .a {
    color: red;
    :has(&) {
      color: green;
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
