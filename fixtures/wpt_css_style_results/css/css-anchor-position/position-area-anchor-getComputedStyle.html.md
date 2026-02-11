# css/css-anchor-position/position-area-anchor-getComputedStyle.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-anchor-getComputedStyle.html"
}
```

## style[0]

```css

.abs-cb {
  width: 200px;
  height: 200px;
  border: 5px solid;
  position: relative;
}

.anchor {
  anchor-name: --a;
  width: 50px;
  height: 50px;
  margin-left: 75px;
  background: pink;
}

.positioned {
  position: absolute;
  position-anchor: --a;
  width: 25px;
  height: 25px;
  background: purple;
}

.top {
  bottom: anchor(top);
}

.bottom {
  top: anchor(bottom);
}

.left {
  right: anchor(left);
}

.right {
  left: anchor(right);
}

.bottom.right {
  position-area: bottom right;
}

.bottom.left {
  position-area: bottom left;
}

.top.right {
  position-area: top right;
}

.top.left {
  position-area: top left;
}
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
      "message": "Invalid value for property “background”.",
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
