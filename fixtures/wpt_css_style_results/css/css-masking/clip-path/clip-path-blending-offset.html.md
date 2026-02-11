# css/css-masking/clip-path/clip-path-blending-offset.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-blending-offset.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
}
#clip-path {
  overflow: hidden;
  background: green;
  clip-path: polygon(0 0, 100px 0, 100px 30px, 30px 30px, 30px 100px, 0 100px);
}
#blend {
  position: absolute;
  mix-blend-mode: multiply;
  left: 40px;
  top: 50px;
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
