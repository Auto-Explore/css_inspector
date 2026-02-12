# css/css-anchor-position/anchor-position-multicol-006.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-multicol-006.html"
}
```

## style[0]

```css

.relpos {
  position: relative;
}
.columns {
  column-count: 3;
  column-fill: auto;
  column-gap: 10px;
  column-width: 100px;
  width: 320px;
  height: 50px;
}
.anchor1 {
  anchor-name: --a1;
  position: absolute;
  width: 10px;
  height: 30px;
  background: orange;
}
.anchor2 {
  anchor-name: --a2;
  position: absolute;
  width: 30px;
  height: 30px;
  background: purple;
}
.target {
  position: absolute;
  width: 5px;
  background: lime;
}
.target1 {
  left: anchor(--a1 left);
  top: anchor(--a1 top);
  height: anchor-size(--a1 height);
}
.target2 {
  left: anchor(--a2 left);
  top: anchor(--a2 top);
  height: anchor-size(--a2 height);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
