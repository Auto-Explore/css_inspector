# css/css-anchor-position/anchor-scroll-composited-scrolling-005-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-composited-scrolling-005-crash.html"
}
```

## style[0]

```css

#scroller {
  margin-top: 200px;
  height: 200px;
  overflow-y: hidden;
}

#anchor {
  anchor-name: --a;
}

#spacer {
  height: 400px;
}

#target {
  position: fixed;
  top: anchor(bottom);
  left: anchor(left);
  position-anchor: --a;
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
