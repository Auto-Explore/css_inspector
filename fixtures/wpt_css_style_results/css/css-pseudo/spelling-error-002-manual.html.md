# css/css-pseudo/spelling-error-002-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/spelling-error-002-manual.html"
}
```

## style[0]

```css

  input
    {
      font-size: 300%;
    }

  input::spelling-error
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
