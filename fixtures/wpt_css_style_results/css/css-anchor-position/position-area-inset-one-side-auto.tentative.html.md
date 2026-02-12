# css/css-anchor-position/position-area-inset-one-side-auto.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-inset-one-side-auto.tentative.html"
}
```

## style[0]

```css

.abs {
  width: 200px;
  height: 200px;
  position: relative;
  border: 1px solid;
}

.anchor {
  width: 100px;
  height: 100px;
  background: tomato;
  anchor-name: --a;
  margin: 100px 0 0 100px;
}

.ref {
  position: absolute;
  inset: 100px auto auto 100px;
  width: 50px;
  height: 50px;
  background: green;
}

.positioned {
  inset: 100% auto auto 100px;
  width: 50px;
  height: 50px;
  position-area: top left;
  background: skyblue;
  position-anchor: --a;
  position: absolute;
  position-visibility: always;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
