# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-contain-intrinsic-size-007.html"
}
```

## style[0]

```css

#border {
  border: 1px solid blue;
  width: max-content;
}

#target {
  display: grid-lanes;
  background: lightblue;
  contain-intrinsic-size: 55px 66px;
  contain: size;
  border-style: solid;
  border-color: black;
  border-width: 2px 3px 5px 7px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
