# css/css-anchor-position/anchor-scroll-update-008.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-update-008.html"
}
```

## style[0]

```css

.abspos-cb {
  position: relative;
  width: 200px;
  height: 200px;
  border: 1px solid;
}

.positioned {
  width: 15px;
  height: 15px;
  background: purple;

  position: absolute;
  position-anchor: --a;
  left: anchor(right);
  top: anchor(top);
}

.scroller {
  overflow-y: scroll;
  height: 100%;
}

.filler {
  width: 1px;
  height: 500px;
}

.anchor {
  width: 15px;
  height: 15px;
  background: magenta;
  anchor-name: --a;
}

.contain {
  contain: layout size;
  width: 200px;
  height: 200px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
