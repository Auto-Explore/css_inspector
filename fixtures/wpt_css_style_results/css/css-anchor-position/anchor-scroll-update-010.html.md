# css/css-anchor-position/anchor-scroll-update-010.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-update-010.html"
}
```

## style[0]

```css

.anchor {
  width: 20px;
  height: 20px;
  background: magenta;
}

.positioned {
  position-anchor: --a;
  position: absolute;
  background: purple;
  width: 20px;
  height: 20px;
  /* Initially not part of --a's nearest scroll container */
  left: anchor(--b right);
  top: anchor(--b top);
}

.abs-cb {
  position: relative;
  width: 200px;
  height: 200px;
  border: 1px solid;
}

.scroll {
  overflow: scroll;
}

.outer {
  width: 200px;
  height: 200px;
}

.inner {
  width: 150px;
  height: 150px;
}

.filler {
  width: 1px;
  height: 200px;
}

.contain {
  contain: layout size;
  width: 200px;
  height: 200px;
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
      "message": "Unknown property “position-anchor”.",
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
