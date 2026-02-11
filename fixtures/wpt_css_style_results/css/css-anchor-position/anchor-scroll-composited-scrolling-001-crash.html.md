# css/css-anchor-position/anchor-scroll-composited-scrolling-001-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-composited-scrolling-001-crash.html"
}
```

## style[0]

```css

#container {
  position: relative;
}

#scroller {
  overflow: scroll;
  width: 400px;
  height: 400px;
}

#anchor {
  anchor-name: --a;
  margin: 100px;
  width: 100px;
  height: 100px;
  background: green;
}

#anchored {
  position: absolute;
  position-anchor: --a;
  left: anchor(left);
  bottom: anchor(top);
  width: 100px;
  height: 100px;
  background: orange;
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
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
