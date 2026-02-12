# css/css-transforms/transform-box/fill-box-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-box/fill-box-001.html"
}
```

## style[0]

```css

svg {
  background-color: red;
}
rect {
  transform-box: fill-box;
}
#target1 {
  transform: rotate(90deg);
}
#target2 {
  transform: translate(50%, -50%);
}
#target3 {
  transform-origin: 25% 25%;
  transform: rotate(180deg) translate(-25%, -25%);
}
#target4 {
  transform-origin: 75px 75px;
  transform: rotate(-180deg) translate(-25%, -25%);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
