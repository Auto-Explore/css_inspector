# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-026.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-026.html"
}
```

## style[0]

```css

.grid {
  display: inline-grid;
  border: 1px solid black;
  grid-template-columns: repeat(auto-fit, 100px);
  height: 40px;
}
.one {
  contain-intrinsic-size: 100px;
  contain: size;
  min-width: 200px;
}
.two {
  contain-intrinsic-size: 200px;
  contain: size;
  min-width: 100px;
}
.three {
  contain-intrinsic-size: 100px;
  contain: size;
  min-width: 200px;
  max-width: 150px;
}
.four {
  contain-intrinsic-size: 200px;
  contain: size;
  min-width: 100px;
  max-width: 150px;
}
.item {
  background: green;
  height: 100%;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
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
