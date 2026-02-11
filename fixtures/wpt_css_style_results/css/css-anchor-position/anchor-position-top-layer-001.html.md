# css/css-anchor-position/anchor-position-top-layer-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-top-layer-001.html"
}
```

## style[0]

```css

#anchor {
  position: absolute;
  top: 300px;
  left: 200px;
  width: 100px;
  height: 100px;
  background: orange;
  anchor-name: --a;
}

#target {
  top: anchor(top);
  left: anchor(right);
  width: 100px;
  height: 100px;
  background: lime;
  position-anchor: --a;
  outline: none;
}

body {
  margin: 0;
  height: 300vh;
}

dialog {
  margin: 0;
  border: 0;
  padding: 0;
  inset: auto;
}

dialog::backdrop {
  background: transparent;
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
