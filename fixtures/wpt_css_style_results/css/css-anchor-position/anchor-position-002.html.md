# css/css-anchor-position/anchor-position-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-002.html"
}
```

## style[0]

```css

#container {
  position: relative;
  transform: translate(0, 0);  /* Make it a containing block. */
}
#anchor1 {
  anchor-name: --a1;
  width: 5px;
  height: 7px;
  background: orange;
}
#anchor2 {
  anchor-name: --a2;
  width: 9px;
  height: 11px;
  background: blue;
}
#anchor3 {
  anchor-name: --a3;
  width: 13px;
  height: 15px;
  background: purple;
}
.target {
  position: absolute;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
