# css/css-anchor-position/anchor-name-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-name-003.html"
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
.anchor1 {
  anchor-name: --a1;
  width: 10px;
  height: 10px;
  background: orange;
}
.target {
  position: absolute;
  width: anchor-size(--a1 width);
  height: 10px;
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
