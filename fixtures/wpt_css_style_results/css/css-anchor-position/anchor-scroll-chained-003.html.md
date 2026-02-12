# css/css-anchor-position/anchor-scroll-chained-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-chained-003.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

div {
  width: 100px;
  height: 100px;
}

#scroller1 {
  width: 200px;
  height: 200px;
  position: relative;
}

#scroller1,#scroller2 {
  overflow: scroll;
}

#anchor {
  anchor-name: --a1;
  height: 20px;
  background: orange;
}

#anchored1 {
  position: absolute;
  position-anchor: --a1;
  left: anchor(left);
  top: anchor(bottom);
  background: green;
  anchor-name: --a2;
  z-index: 1;
}

#anchored2 {
  position: absolute;
  position-anchor: --a2;
  left: anchor(left);
  top: anchor(bottom);
  background: lime;
  z-index: 1;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
