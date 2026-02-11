# css/css-values/ch-unit-010.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ch-unit-010.html"
}
```

## style[0]

```css

  div
    {
      float: left;
      font-size: 80px; /* arbitrary font size */
      text-orientation: mixed;
      writing-mode: vertical-rl;
    }

  div#test-blue
    {
      background-color: blue;
      height: 5ch;
      width: 1.8em;
    }

  div#reference-orange
    {
      background-color: orange;
      color: orange;
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
