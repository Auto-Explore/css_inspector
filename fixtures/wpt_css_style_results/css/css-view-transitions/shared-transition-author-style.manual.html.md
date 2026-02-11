# css/css-view-transitions/shared-transition-author-style.manual.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/shared-transition-author-style.manual.html"
}
```

## style[0]

```css

body {
  background: lightpink;
  overflow: hidden;
}

input {
  position: absolute;
  left: 8px;
  top: 8px;
  z-index: 10;
}

.top {
  top: 0px;
  background: green;
}
.bottom {
  bottom: 0px;
  background: blue;
}

div {
  position: absolute;
  left: 0px;
  right: 0px;
  height: 40vh;
  contain: paint;
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
