# css/css-view-transitions/new-content-captures-different-size-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-captures-different-size-ref.html"
}
```

## style[0]

```css

.box {
  color: red;
  background: lightgreen;
  width: 100px;
  height: 100px;
  contain: paint;
  position: absolute;
  font-size: 30pt;
}
#e1 {
  clip-path: circle(30%);
  top: 20px;
  left: 20px;
}
#e2 {
  clip-path: ellipse(70% 30%);
  top: 160px;
  left: 20px;
}
#e3 {
  filter: blur(5px);
  top: 300px;
  left: 20px;
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
