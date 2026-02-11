# css/css-transitions/pseudo-element-transform-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/pseudo-element-transform-ref.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: rgb(255, 191, 0);
}
div::before {
  content: "";
  background: rgb(184, 115, 51);
  width: 100px;
  height: 100px;
  transform: ScaleX(0.5);
  display: block;
  will-change: transform;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
