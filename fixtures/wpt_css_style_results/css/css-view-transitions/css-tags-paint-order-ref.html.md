# css/css-view-transitions/css-tags-paint-order-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/css-tags-paint-order-ref.html"
}
```

## style[0]

```css

div {
  position: absolute;
  top: 50px;
  width: 100px;
  height: 100px;
}
#one {
  background: green;
  left: 50px;
  z-index: 1;
}
#two {
  background: yellow;
  left: 125px;
  z-index: -1;
}
#three {
  background: blue;
  left: 200px;
  z-index: 0;
}
body { background: lightpink; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
