# css/css-anchor-position/anchor-scroll-to-sticky-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-to-sticky-002.html"
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

#scroller {
  overflow: scroll;
  position: relative;
}

#anchor {
  anchor-name: --a1;
  position: sticky;
  top: 0;
  height: 20px;
  background: orange;
}

#anchored {
  position: absolute;
  position-anchor: --a1;
  left: anchor(left);
  top: anchor(bottom);
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
