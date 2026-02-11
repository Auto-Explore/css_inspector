# css/css-anchor-position/anchor-scroll-to-sticky-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-to-sticky-004.html"
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

.sticky {
  position: sticky;
  top: 0;
}

#anchor {
  anchor-name: --a1;
  height: 20px;
  background: orange;
}

#anchored {
  position: absolute;
  position-anchor: --a1;
  left: anchor(left);
  top: anchor(bottom);
  background: green;
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
      "message": "Invalid value for property “background”.",
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
