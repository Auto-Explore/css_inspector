# css/css-pseudo/svg-text-selection-fill-only.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/svg-text-selection-fill-only.html"
}
```

## style[0]

```css

  div {
    font: 16px Ahem;
  }
  text {
    fill: red;
    stroke: darkgreen;
  }
  ::selection {
    background-color: transparent; /* Avoid default background color. */
    fill: lightgreen;
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
      "message": "Unknown property “fill”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
