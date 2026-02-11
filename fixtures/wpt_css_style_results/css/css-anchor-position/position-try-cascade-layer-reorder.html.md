# css/css-anchor-position/position-try-cascade-layer-reorder.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-cascade-layer-reorder.html"
}
```

## style[0]

```css

body { margin: 0; }
#anchor {
  width: 100px;
  height: 100px;
  margin-left: 200px;
  margin-top: 200px;
  color: orange;
  anchor-name: --a;
}

.target {
  position: absolute;
  width: 100px;
  height: 100px;
  color: lime;
  position-try-fallbacks: --fallback;
  left: 999999px; /* force fallback */
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
