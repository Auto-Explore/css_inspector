# css/css-values/attr-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-invalidation.html"
}
```

## style[0]

```css

  div {
    width: attr(data-foo type(<length>));
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
