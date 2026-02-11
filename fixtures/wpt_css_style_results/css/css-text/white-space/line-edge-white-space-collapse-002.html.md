# css/css-text/white-space/line-edge-white-space-collapse-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/line-edge-white-space-collapse-002.html"
}
```

## style[0]

```css

div { font: 30px/30px monospace; }
span span { border-left: 30px solid green }
aside {
  font: 30px/30px monospace;
  width: 30px;
  background: red;
  position: absolute;
  z-index:-1;
  height: 300px;

  /* to avoid accidental bleeding at the edges by a pixel or a sub pixel*/
  box-sizing: border-box;
  border: solid white 5px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
