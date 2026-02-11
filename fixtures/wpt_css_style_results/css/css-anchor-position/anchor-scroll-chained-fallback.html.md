# css/css-anchor-position/anchor-scroll-chained-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-chained-fallback.html"
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
  width: 300px;
  height: 300px;
}

#anchor {
  anchor-name: --a1;
  background: orange;
  margin: 250px 0;
}

#anchored1 {
  position: absolute;
  position-anchor: --a1;
  background: green;
  position-try-fallbacks: --fallback;
  anchor-name: --a2;
  left: anchor(left);
  bottom: anchor(top);
}

#anchored2 {
  position: absolute;
  position-anchor: --a2;
  left: anchor(left);
  top: anchor(bottom);
  background: lime;
}

@position-try --fallback {
  left: anchor(right);
  top: anchor(top);
  bottom: auto;
}

```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
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
      "message": "Unknown property “position-try-fallbacks”.",
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
