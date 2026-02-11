# css/css-pseudo/grammar-error-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/grammar-error-001.html"
}
```

## style[0]

```css

  div
    {
      font-size: 300%;
    }

  div::grammar-error
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
