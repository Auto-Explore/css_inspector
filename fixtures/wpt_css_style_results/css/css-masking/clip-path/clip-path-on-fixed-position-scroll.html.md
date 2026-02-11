# css/css-masking/clip-path/clip-path-on-fixed-position-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-on-fixed-position-scroll.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
}
.clip {
  clip-path: inset(0);
}
.fixed {
  position: fixed;
  background: red;
  z-index: -1;
  bottom: 0;
}
.green {
  position: relative;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
