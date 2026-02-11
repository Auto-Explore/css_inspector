# css/css-nesting/pseudo-part-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/pseudo-part-crash.html"
}
```

## style[0]

```css

  div::part(x) {
    & {
      color: red;
    }
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
