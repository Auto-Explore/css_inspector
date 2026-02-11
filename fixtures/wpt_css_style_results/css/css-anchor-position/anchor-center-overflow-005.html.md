# css/css-anchor-position/anchor-center-overflow-005.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-overflow-005.html"
}
```

## style[0]

```css

.abs-cb {
  display: inline-block;
  box-sizing: border-box;
  border: 5px solid;
  width: 100px;
  height: 100px;
  anchor-scope: --a;
  position: relative;
}

.anchor {
  anchor-name: --a;
  width: 20px;
  height: 20px;
  background: pink;
  position: absolute;
  inset: 0;
}

.positioned {
  position: absolute;
  width: 50px;
  height: 50px;
  margin: 5px;
  position-anchor: --a;
  background: purple;
}

.anchor.tl {
  align-self: start;
  justify-self: start;
}

.anchor.tr {
  align-self: start;
  justify-self: end;
}

.anchor.bl {
  align-self: end;
  justify-self: start;
}

.anchor.br {
  align-self: end;
  justify-self: end;
}

.positioned.bc {
  top: anchor(bottom);
  justify-self: anchor-center;
}

.positioned.tc {
  bottom: anchor(top);
  justify-self: anchor-center;
}

.positioned.cr {
  left: anchor(right);
  align-self: anchor-center;
}

.positioned.cl {
  right: anchor(left);
  align-self: anchor-center;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
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
    }
  ],
  "warnings": 0
}
```
