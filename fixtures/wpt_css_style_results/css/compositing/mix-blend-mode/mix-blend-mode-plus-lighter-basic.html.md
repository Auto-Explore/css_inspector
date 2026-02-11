# css/compositing/mix-blend-mode/mix-blend-mode-plus-lighter-basic.html

```json
{
  "format_version": 3,
  "file": "css/compositing/mix-blend-mode/mix-blend-mode-plus-lighter-basic.html"
}
```

## style[0]

```css

.container {
  position: relative;
  isolation: isolate;
  width: 500px;
  height: 500px;
}
.blue { background: #000064; }
.green { background: #006400; }
.common {
  position: absolute;
  width: 100px;
  height: 100px;
  opacity: 0.6;
}
.one {
  top: 10px;
  left: 10px;
}
.two {
  top: 65px;
  left: 30px;
  mix-blend-mode: plus-lighter;
}
.three {
  top: 120px;
  left: 50px;
  mix-blend-mode: plus-lighter;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
