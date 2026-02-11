# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-025-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-025-ref.html"
}
```

## style[0]

```css

.grid {
  border: 3px solid black;
  display: grid;
  width: 70px;
  height: 80px;
  box-sizing: content-box;
  background: lightblue;
  grid-gap: 5px;
}
.one {
  grid-template: repeat(8, 10px) / 3fr 4fr;
}
.two {
  grid-template: 1fr 2fr / repeat(3, 15px);
}
.three {
  grid-template: repeat(8, 10px) / repeat(3, 15px);
}

.item {
  background: green;
  height: 100%;
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
