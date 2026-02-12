# css/css-anchor-position/anchor-scroll-cleanup.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-cleanup.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.scroller {
  width: 100px;
  height: 100px;
  overflow-y: scroll;
}

.nonpos-cb {
  transform: scale(1);
}

.abspos-cb {
  position: absolute;
}

.anchor {
  background: orange;
  anchor-name: --a1;
  position: absolute;
  width: 50px;
  height: 50px;
  top: 50px;
}

.spacer {
  height: 200px;
}

.target {
  background: lime;
  position: absolute;
  width: 50px;
  height: 50px;
  top: anchor(top);
  left: anchor(right);
  position-anchor: --a1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
