# css/css-anchor-position/anchor-center-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-003.html"
}
```

## style[0]

```css

.container {
  width: 100px;
  height: 100px;
  border: solid 3px;
  position: relative;
  margin: 50px;
}

.anchor {
  anchor-name: --anchor;
  position: relative;
  width: 50px;
  height: 50px;
  left: 40px;
  top: 5px;
  background: lime;
}

.target {
  position-anchor: --anchor;
  position: fixed;
  background: cyan;
  justify-self: anchor-center;
  top: anchor(bottom);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
