# css/css-view-transitions/clip-path-larger-than-border-box-on-child-of-named-element-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/clip-path-larger-than-border-box-on-child-of-named-element-ref.html"
}
```

## style[0]

```css

body {
  background: lightpink;
}

.target {
  width: 100px;
  height: 100px;
  background: blue;
}
.child {
  width: 10px;
  height: 10px;
  position: relative;
  top: 100px;
  left: 100px;
  background: green;
  clip-path: inset(-10px -100px -50px -20px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
