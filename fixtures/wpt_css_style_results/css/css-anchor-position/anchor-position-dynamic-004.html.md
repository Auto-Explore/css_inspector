# css/css-anchor-position/anchor-position-dynamic-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-dynamic-004.html"
}
```

## style[0]

```css

.cb {
  position: relative;
}
#anchor1 {
  anchor-name: --a1;
  margin-left: 15px;
  width: 30px;
  height: 20px;
  background: red;
}
.after #anchor1 {
  margin-left: 50px;
}
.target {
  position: absolute;
  left: anchor(--a1 left);
  top: anchor(--a1 top);
  right: anchor(--a1 right);
  bottom: anchor(--a1 bottom);
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
