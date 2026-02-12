# css/css-anchor-position/anchor-scroll-composited-scrolling-002-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-composited-scrolling-002-crash.html"
}
```

## style[0]

```css

#scroller {
  margin-top: 200px;
  height: 200px;
  overflow-y: scroll;
}

#anchor {
  anchor-name: --a;
}

#spacer {
  height: 400px;
}

#target {
  position: fixed;
  top: anchor(bottom);
  left: anchor(left);
  position-anchor: --a;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
