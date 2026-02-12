# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-025.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-025.html"
}
```

## style[0]

```css

.grid {
  border: 3px solid black;
  display: grid;
  contain-intrinsic-size: 70px 80px;
  contain: size;
  width: max-content;
  background: lightblue;
  grid-gap: 5px;
}
.one {
  grid-template: repeat(auto-fit, 10px) / 3fr 4fr;
}
.two {
  grid-template: 1fr 2fr / repeat(auto-fit, 15px);
}
.three {
  grid-template: repeat(auto-fit, 10px) / repeat(auto-fit, 15px);
}

.item {
  background: green;
  height: 100%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
