# css/css-grid/layout-algorithm/grid-flex-track-intrinsic-sizes-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/layout-algorithm/grid-flex-track-intrinsic-sizes-002.html"
}
```

## style[0]

```css

#grid {
  display: grid;
  grid-template-areas: ". . . ."
                       ". a . ."
                       ". . . ."
                       ". . . b";
  width: 50px;
  height: 50px;
  border: solid;
}
#item1 {
  grid-column: 1 / a;
  grid-row: 1 / a;
  width: 60px;
  height: 60px;
  background: blue;
}
#item2 {
  grid-column: a / b;
  grid-row: a / b;
  width: 150px;
  height: 150px;
  background: yellow;
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “grid-template-areas”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-row”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
