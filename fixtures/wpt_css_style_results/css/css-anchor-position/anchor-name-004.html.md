# css/css-anchor-position/anchor-name-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-004.html"
}
```

## style[0]

```css

.relpos {
  position: relative;
}
.anchor1 {
  anchor-name: --a1, --a2;
  width: 30px;
  height: 10px;
  background: orange;
}
.target {
  position: absolute;
  height: 10px;
  background: lime;
}
#target1 {
  width: anchor-size(--a1 width);
}
#target2 {
  width: anchor-size(--a2 width);
}
#target3 {
  width: anchor-size(--a3 width, 11px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
