# css/css-grid/grid-lanes/tentative/track-sizing/grid-lanes-track-sizing-check-grid-height-on-resize.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/grid-lanes-track-sizing-check-grid-height-on-resize.html"
}
```

## style[0]

```css

  grid {
    display: inline-grid-lanes;
    grid-template-columns: auto;
    grid-gap: 10px;
    border: 10px;
    border-style: solid;
  }
  item {
    background-color: grey;
    height: 250px;
    width: 250px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
