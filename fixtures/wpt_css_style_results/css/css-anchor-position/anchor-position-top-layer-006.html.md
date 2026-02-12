# css/css-anchor-position/anchor-position-top-layer-006.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-top-layer-006.html"
}
```

## style[0]

```css

#anchor {
  position: absolute;
  top: 300px;
  left: 200px;
  width: 100px;
  height: 100px;
  background: orange;
  anchor-name: --a;
}

#target {
  position: fixed;
  top: anchor(bottom, 200px);
  left: anchor(left, 300px);
  width: 100px;
  height: 100px;
  background: lime;
  position-anchor: --a;
}

body {
  margin: 0;
  height: 300vh;
}

dialog {
  margin: 0;
  border: 0;
  padding: 0;
  outline: none;
}

dialog::backdrop {
  background: transparent;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
