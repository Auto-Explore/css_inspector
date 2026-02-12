# css/css-anchor-position/anchor-scroll-position-try-012.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-position-try-012.html"
}
```

## style[0]

```css

#scroller {
  position: relative;
  width: 400px;
  height: 400px;
  overflow-y: scroll;
}

.box {
  min-height: 100px;
  width: 100px;
}

#anchor {
  anchor-name: --a;
  background: orange;
}

#anchored {
  position: absolute;
  top: anchor(bottom);
  width: 100px;
  height: 100px;
  background: green;
  position-anchor: --a;
  position-try-fallbacks: --pf;
}

@position-try --pf {
  top: auto;
  bottom: anchor(top);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
