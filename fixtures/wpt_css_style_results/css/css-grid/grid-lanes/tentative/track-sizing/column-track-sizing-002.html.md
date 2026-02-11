# css/css-grid/grid-lanes/tentative/track-sizing/column-track-sizing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/column-track-sizing-002.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-template-columns: minmax(15px, min-content) max-content auto;
    grid-lanes-direction: normal;
    background-color: gray;
    width: 100px;
    font: 10px/1 Ahem;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
