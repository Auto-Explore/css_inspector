# css/css-nesting/invalidation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/invalidation-001.html"
}
```

## style[0]

```css

  .b {
    color: red;
  }
  .a {
    & .b {
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
