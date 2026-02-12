# css/css-anchor-position/no-anchor-anchor-center.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/no-anchor-anchor-center.html"
}
```

## style[0]

```css

.anchor {
  anchor-name: --dropdownAnchor;
  background: orange;
  width: 100px;
  background-color: orange;
}
.container {
  position: fixed;
  top: 20px;
  left: 50px;
  width: 300px;
  height: 200px;
  background: green;
}
.target {
  position-anchor: --dropdownAnchor;
  display: flow;
  left: 10px;
  right: 10px;
  position: absolute;
  justify-self: anchor-center;
  width: 80px;
  border: 1px #000 solid;
  background: lime;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
