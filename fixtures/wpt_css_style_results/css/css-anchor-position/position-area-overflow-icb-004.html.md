# css/css-anchor-position/position-area-overflow-icb-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-overflow-icb-004.html"
}
```

## style[0]

```css

.anchor {
  anchor-name: --foo;
  border: solid orange 20px;
  position: absolute;
  margin: auto;
  top: 50%;
  left: 50%;
}
.above, .below, .left, .right {
  border: solid blue 3px;
  padding: 2px;
  position: fixed;
  position-anchor: --foo;
}
.above { position-area: top center;    height: 60vh;  margin-left:   20px; }
.below { position-area: bottom center; height: 60vh;  margin-right:  20px; }
.left  { position-area: left center;   width:  60vw;  margin-top:    20px; }
.right { position-area: right center;  width:  60vw;  margin-bottom: 20px; }

html { height: 150vh; width: 150vw; }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
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
