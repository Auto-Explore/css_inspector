# css/css-anchor-position/position-area-in-grid.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-in-grid.html"
}
```

## style[0]

```css

  #container {
    display: grid;
    grid: 1fr 1fr 1fr 1fr / 1fr 1fr 1fr 1fr;
    position: relative;
    width: 400px;
    height: 400px;
    outline: 1px solid;
  }

  #anchor {
    position: absolute;
    left: 100px;
    top: 150px;
    width: 150px;
    height: 75px;
    anchor-name: --anchor;
    background: blue;
  }

  #anchored {
    grid-row-start: 2;
    grid-row-end: span 2;
    grid-column-start: 3;
    grid-column-end: auto;
    position: absolute;
    align-self: stretch;
    justify-self: stretch;
    position-anchor: --anchor;
    border: solid orange;
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row-end”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
