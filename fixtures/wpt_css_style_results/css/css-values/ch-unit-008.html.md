# css/css-values/ch-unit-008.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ch-unit-008.html"
}
```

## style[0]

```css

  div
    {
      font-size: 80px; /* arbitrary font size */
    }

  div#test-blue
    {
      background-color: blue;
      height: 1.8em;
      width: 5ch;
    }

  div#reference-orange
    {
      background-color: orange;
      color: orange;
      float: left;
      line-height: 1.8; /* arbitrary line-height */
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
