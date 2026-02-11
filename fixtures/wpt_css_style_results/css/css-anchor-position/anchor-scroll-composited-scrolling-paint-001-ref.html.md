# css/css-anchor-position/anchor-scroll-composited-scrolling-paint-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-composited-scrolling-paint-001-ref.html"
}
```

## style[0]

```css

.abs-cb {
  width: 100px;
  height: 100px;
  position: relative;
}

.anchor {
  anchor-name: --a;
  width: 50px;
  height: 50px;
  background: blue;
}

.chain {
  width: 25px;
  height: 25px;
  background: pink;
  position: absolute;
  left: 50px;
  top: 50px;
}

.positioned {
  width: 25px;
  height: 25px;
  background: yellow;
  position: absolute;
  left: 75px;
  top: 75px;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
