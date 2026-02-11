# css/css-nesting/invalidation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/invalidation-002.html"
}
```

## style[0]

```css

  .a {
    color: green;
  }
  .a {
    & .b {
      color: red;
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
