# css/css-anchor-position/anchor-scroll-005.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-005.html"
}
```

## style[0]

```css

body {
  margin: 0;
  height: 200vh;
}

#scroller {
  position: fixed;
  width: 200px;
  height: 200px;
  overflow-y: scroll;
}

#anchor {
  position: absolute;
  width: 100px;
  height: 100px;
  top: 200px;
  background: orange;
  anchor-name: --a;
}

#target {
  position: fixed;
  width: 100px;
  height: 100px;
  bottom: anchor(top);
  position-anchor: --a;
  background: lime;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
