# css/css-position/fixed-z-index-blend.html

```json
{
  "format_version": 3,
  "file": "css/css-position/fixed-z-index-blend.html"
}
```

## style[0]

```css

.blend {
  display: block;
  position: fixed;
  z-index: 2;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  mix-blend-mode: overlay;
}

.background {
  pointer-events: none;
  position: fixed;
  z-index: 1;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  opacity: 1;
}

.text {
  position: relative;
  z-index: 3;
  overflow: hidden;
  width: 100vw;
  min-height: 100vh;
  font-size: 50px;
  line-height: 2;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “pointer-events”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
