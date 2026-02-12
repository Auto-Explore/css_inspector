# css/css-transforms/perspective-split-by-zero-w.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-split-by-zero-w.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
}
html { overflow: hidden }
body {
  perspective: 500px;
  perspective-origin: 400px 299px;
  background: rgb(200, 200, 200);
}
div {
  width: 1140px;
  height: 990px;
  transform-style: preserve-3d;
  position: absolute;
  top: 299.5px;
  left: 400px;
  transform: translate3d(-570px, -495px, 500px) rotateY(64.24deg) translateY(23px) rotateX(90deg);
  background-image: url('support/tile-bg.png');
  background-size: 100% 100%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
