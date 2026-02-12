# css/css-anchor-position/anchor-position-multicol-005.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-multicol-005.html"
}
```

## style[0]

```css

.relpos {
  position: relative;
}
.abspos {
  position: absolute;
}
.columns {
  column-count: 2;
  column-fill: auto;
  column-gap: 10px;
  column-width: 100px;
  width: 210px;
  height: 100px;
}
.spacer {
  height: 10px;
  background: pink;
}
.anchor1 {
  anchor-name: --a1;
  margin-left: 10px;
  width: 40px;
  height: 80px;
  background: orange;
}
.target {
  position: absolute;
  background: lime;
  opacity: 1;
}
.target1 {
  left: anchor(--a1 left);
  top: anchor(--a1 top);
  width: anchor-size(--a1 width);
  height: anchor-size(--a1 height);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
