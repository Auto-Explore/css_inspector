# css/css-anchor-position/reference/anchor-scroll-composited-scrolling-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/reference/anchor-scroll-composited-scrolling-006-ref.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
#scroller {
  width: 200px;
  height: 100px;
  overflow: scroll;
  will-change: scroll-position;
}
#spacer {
  height: 400px;
}
#anchor {
  width: 100px;
  height: 100px;
  anchor-name: --a;
}
#overlap {
  position: absolute;
  width: 100px;
  height: 100px;
  top: 150px;
  left: 0;
  z-index: 100;
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
