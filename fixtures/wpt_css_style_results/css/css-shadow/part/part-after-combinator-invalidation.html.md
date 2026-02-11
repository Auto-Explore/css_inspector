# css/css-shadow/part/part-after-combinator-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/part-after-combinator-invalidation.html"
}
```

## style[0]

```css

.inactive > ::part(content) {
  color: red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
