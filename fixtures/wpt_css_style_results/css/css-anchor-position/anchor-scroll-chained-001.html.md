# css/css-anchor-position/anchor-scroll-chained-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-chained-001.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

div {
  width: 100px;
  height: 100px;
}

#scroller {
  overflow: scroll;
}

#anchor {
  anchor-name: --a1;
  height: 20px;
  background: orange;
}

#anchored1 {
  position: absolute;
  position-anchor: --a1;
  left: anchor(left);
  top: anchor(bottom);
  background: green;
  anchor-name: --a2;
}

#anchored2 {
  position: absolute;
  position-anchor: --a2;
  left: anchor(left);
  top: anchor(bottom);
  background: lime;
}

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
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
