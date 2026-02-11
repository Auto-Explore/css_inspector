# css/css-anchor-position/anchor-position-top-layer-007.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-top-layer-007.html"
}
```

## style[0]

```css

#container {
  position: fixed;
  inset: 0;
}
#anchor {
  position: fixed;
  top: 100px;
  left: 100px;
  width: 50px;
  height: 80px;
  anchor-name: --anchor;
}
#target {
  position: fixed;
  inset: auto;
  top: anchor(--anchor top);
  left: anchor(--anchor right);
  margin: 0;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
