# css/css-pseudo/grammar-error-003-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/grammar-error-003-manual.html"
}
```

## style[0]

```css

  textarea
    {
      font-size: 300%;
    }

  textarea::grammar-error
    {
      color: maroon;
      text-decoration: underline dotted red;
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
