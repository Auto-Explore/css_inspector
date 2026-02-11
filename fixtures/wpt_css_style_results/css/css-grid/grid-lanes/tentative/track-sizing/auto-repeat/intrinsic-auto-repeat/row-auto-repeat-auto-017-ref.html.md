# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-017-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-017-ref.html"
}
```

## style[0]

```css

.grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    grid-template-rows: repeat(9, auto);
    width: 200px;
    height: 500px;
    gap: 10px;
}

.grid-lanes > div {
    height: 50px;
    width: 100px;
    background-color: orange;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
