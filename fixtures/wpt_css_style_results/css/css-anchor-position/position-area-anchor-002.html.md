# css/css-anchor-position/position-area-anchor-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-anchor-002.html"
}
```

## style[0]

```css

.abs-cb {
  width: 180px;
  height: 180px;
  position: relative;
  border: 5px solid;
}

.scroller {
  width: 100%;
  height: 100%;
  overflow: scroll;
}

.anchor {
  width: 60px;
  height: 60px;
  margin-left: 60px;
  anchor-name: --a;
  background: pink;
}

.filler {
  width: 1px;
  height: 180px;
}

.positioned {
  width: 30px;
  height: 30px;
  background: purple;
  position: absolute;
  position-anchor: --a;
}

.tl {
  position-area: top left;
  right: anchor(left);
  bottom: anchor(top);
}

.tr {
  position-area: top right;
  left: anchor(right);
  bottom: anchor(top);
}

.bl {
  position-area: bottom left;
  right: anchor(left);
  top: anchor(bottom);
}

.br {
  position-area: bottom right;
  left: anchor(right);
  top: anchor(bottom);
}

.dn {
  display: none;
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
