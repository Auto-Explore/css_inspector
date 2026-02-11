# css/css-anchor-position/anchor-scroll-composited-scrolling-006.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-composited-scrolling-006.html"
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
#target {
  position: absolute;
  width: 100px;
  height: 100px;
  background: red;
  left: 0;
  bottom: anchor(top);
  position-anchor: --a;
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
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
