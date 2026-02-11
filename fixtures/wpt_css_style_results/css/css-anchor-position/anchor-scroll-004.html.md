# css/css-anchor-position/anchor-scroll-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-004.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.cb {
  position: relative;
  font: 20px/1 Ahem;
}

.scroller {
  display: inline-block;
  overflow-x: scroll;
  width: 160px;
  white-space: nowrap;
}

.anchor {
  anchor-name: --a;
  color: orange;
}

.target {
  position: absolute;
  position-anchor: --a;
  top: anchor(bottom);
  left: anchor(left);
  color: lime;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
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
