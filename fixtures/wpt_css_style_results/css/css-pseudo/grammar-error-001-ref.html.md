# css/css-pseudo/grammar-error-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/grammar-error-001-ref.html"
}
```

## style[0]

```css

  div
    {
      font-size: 300%;
    }

  span
    {
      color: maroon;
      text-decoration: underline dotted fuchsia;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
