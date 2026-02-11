# css/css-anchor-position/position-area-pseudo-element-implicit-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-pseudo-element-implicit-anchor.html"
}
```

## style[0]

```css

body { margin: 0 }
#target  {
    margin-top: 100px;
    margin-left: 50px;
    width: 100px;
    height: 100px;
    background: blue;
}
#target::before, #target::after {
    width: 100px;
    height: 100px;
    position: absolute;
    position-anchor: auto;
}
#target::before {
    position-area: center right;
    background: green;
    content:'';
}
#target::after {
    position-area: bottom center;
    background: green;
    content:'';
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
