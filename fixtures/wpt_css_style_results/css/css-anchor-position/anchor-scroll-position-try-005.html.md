# css/css-anchor-position/anchor-scroll-position-try-005.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-position-try-005.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

#anchor {
  anchor-name: --a;
  width: 100px;
  height: 100px;
  margin-right: 200px;
  background: orange;
}

#spacer {
  width: 1000vw;
  height: 1px;
}

#anchored {
  writing-mode: vertical-rl;
  position: fixed;
  width: 100px;
  height: 100px;
  background: green;
  position-anchor: --a;
  top: anchor(top);
  left: anchor(right);
  position-try-fallbacks: --pf;
}

@position-try --pf {
  left: auto;
  right: anchor(left);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
