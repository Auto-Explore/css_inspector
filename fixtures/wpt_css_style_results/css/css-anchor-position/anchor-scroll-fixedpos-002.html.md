# css/css-anchor-position/anchor-scroll-fixedpos-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-fixedpos-002.html"
}
```

## style[0]

```css

body {
  margin: 0;
  height: 2000px;
}

div {
  width: 100px;
  height: 100px;
}

#anchor {
  position: fixed;
  anchor-name: --a1;
  margin: 300px;
  background: orange;
}

#anchored {
  position: fixed;
  position-anchor: --a1;
  left: anchor(right);
  top: anchor(top);
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
