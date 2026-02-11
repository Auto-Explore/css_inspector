# css/css-anchor-position/anchor-scroll-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-003.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
#multicol {
  column-count: 3;
  column-width: 90px;
  column-gap: 10px;
  width: 300px;
  height: 100px;
}

#cb {
  height: 300px;
  background: yellow;
  position: relative;
}

#spacer {
  height: 110px;
}

#scroller {
  overflow-y: scroll;
  height: 80px;
  background: blue;
}

#anchor {
  anchor-name: --a;
  width: 50px;
  height: 50px;
  background: orange;
  position: relative;
  top: 80px;
}

#target {
  position: absolute;
  left: anchor(left);
  bottom: anchor(top);
  position-anchor: --a;
  width: 50px;
  height: 50px;
  background: lime;
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
