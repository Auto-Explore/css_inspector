# css/css-values/reference/ch-unit-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-values/reference/ch-unit-008-ref.html"
}
```

## style[0]

```css

  div
    {
      float: left;
      font-size: 80px; /* arbitrary font size */
      line-height: 1.8; /* arbitrary line-height */
    }

  div#blue
    {
      background-color: blue;
      color: blue;
    }

  div#orange
    {
      background-color: orange;
      color: orange;
      clear: left;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
