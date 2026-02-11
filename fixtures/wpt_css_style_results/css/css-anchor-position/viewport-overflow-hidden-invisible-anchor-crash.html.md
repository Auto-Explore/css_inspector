# css/css-anchor-position/viewport-overflow-hidden-invisible-anchor-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/viewport-overflow-hidden-invisible-anchor-crash.html"
}
```

## style[0]

```css

html {
  overflow: hidden;
}
.anchor, .element {
  background: green;
  width: 100px;
  height: 100px;
}
.anchor {
  background: red;
  anchor-name: --anchor;
}
.element {
  position: absolute;
  position-anchor: --anchor;
  bottom: anchor(top);
  left: 50px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
