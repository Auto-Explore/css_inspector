# css/css-pseudo/svg-text-selection-stroke-only.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/svg-text-selection-stroke-only.html"
}
```

## style[0]

```css

  div {
    font: 16px Ahem;
  }
  text {
    fill: lightgreen;
    stroke: red;
  }
  ::selection {
    background-color: transparent; /* Avoid default background color. */
    stroke: darkgreen;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “fill”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “stroke”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
