# css/css-anchor-position/anchor-position-circular.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-circular.html"
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

#anchored1 {
  position: absolute;
  position-anchor: --a1;
  left: anchor(left);
  top: anchor(bottom);
  background: orange;
  anchor-name: --a2;
}

#anchored2 {
  position: absolute;
  position-anchor: --a2;
  left: anchor(left);
  top: anchor(bottom);
  background: green;
  anchor-name: --a1;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
