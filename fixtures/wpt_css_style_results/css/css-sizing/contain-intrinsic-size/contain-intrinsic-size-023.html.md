# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-023.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-023.html"
}
```

## style[0]

```css

#grid {
  border: 3px solid black;
  display: grid;
  grid-template: 1fr 2fr / 3fr 4fr;
  contain-intrinsic-size: 70px 80px;
  contain: size;
  width: max-content;
  background: lightblue;
  grid-gap: 5px;
}
.item {
  background: lightgreen;
  height: 100%;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
