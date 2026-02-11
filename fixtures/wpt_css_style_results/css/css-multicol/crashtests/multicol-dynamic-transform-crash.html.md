# css/css-multicol/crashtests/multicol-dynamic-transform-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/crashtests/multicol-dynamic-transform-crash.html"
}
```

## style[0]

```css

#container {
  columns: 2;
  column-fill: auto;
  background: orange;
  height: 50px;
}
#target {
  width: 100px;
  height: 100px;
  background: purple;
  position: absolute;
}
.transform {
  transform: translate(0, 50px);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
