# css/css-grid/parsing/grid-template-rows-intrinsic-auto-repeat-computed-implicit-track.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/parsing/grid-template-rows-intrinsic-auto-repeat-computed-implicit-track.tentative.html"
}
```

## style[0]

```css

#target {
  display: grid-lanes;
  grid-lanes-direction: row;
  height: 1px;
  font-size: 1px;
}
#item {
  grid-row: auto / 1;
  height: 10px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
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
