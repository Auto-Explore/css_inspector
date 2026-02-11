# css/css-anchor-position/position-try-switch-to-fixed-anchor-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-switch-to-fixed-anchor-ref.html"
}
```

## style[0]

```css

body {
  width: 150vw;
  height: 150vh;
}
.anchor {
  width: 50px;
  height: 50px;
  background: orange;
}
#anchor1 {
  position: absolute;
  top: 100px;
  left: 350px;
}
#anchor2 {
  position:fixed;
  right: 0;
  bottom: 0;
}
#anchored {
  position: fixed;
  right: 50px;
  bottom: 50px;
  width: 50px;
  height: 50px;
  background: blue;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
