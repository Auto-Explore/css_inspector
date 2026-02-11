# css/css-values/reference/ch-unit-011-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-values/reference/ch-unit-011-ref.html"
}
```

## style[0]

```css

  div
    {
      float: left;
      font-size: 80px; /* arbitrary font size */
      line-height: 1.8; /* arbitrary line-height */
      text-orientation: upright;
      writing-mode: vertical-rl;
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
