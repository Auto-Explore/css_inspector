# css/css-masking/clip-path/svg-clip-path-fixed-values-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/svg-clip-path-fixed-values-ref.html"
}
```

## style[0]

```css

svg {
  width: 300px;
  height: 300px;
}
.container {
  width: 300px;
  height: 300px;
  position: relative;
}
.bg1 {
  background: green;
  position: absolute;
  width: 100px;
  height: 100px;
}
.noclip {
  top: 100px;
  left: 100px;
}
.cp1 {
  clip-path: polygon(0px 0px, 100px 0px, 0px 100px);
  top: 0;
  left: 0;
}
.cp2 {
  clip-path: circle(50px);
  top: 200px;
  left: 0;
}
.cp3 {
  clip-path: ellipse(50px 30px);
  top: 200px;
  left: 200px;
}
.cp4 {
  clip-path: inset(10px 20px 30px 40px);
  top: 0;
  left: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
