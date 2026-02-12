# css/css-anchor-position/position-try-switch-from-fixed-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-switch-from-fixed-anchor.html"
}
```

## style[0]

```css

html {
  scrollbar-width: none;
}
body {
  width: 150vw;
  height: 150vh;
}
.anchor {
  width: 50px;
  height: 50px;
  background: orange;
}
#anchor1 {
  anchor-name: --anchor1;
  position: absolute;
  top: 100px;
  left: 350px;
}
#anchor2 {
  anchor-name: --anchor2;
  position:fixed;
  right: 0;
  bottom: 0;
}
#anchored {
  position-anchor: --anchor1;
  position-area: top;
  position-try-fallbacks: --fixed;
  position: fixed;
  width: 50px;
  height: 50px;
  background: blue;
}
@position-try --fixed {
  position-area: top left;
  position-anchor: --anchor2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
