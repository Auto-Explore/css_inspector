# css/css-view-transitions/view-transition-name-is-backdrop-filter-root.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-name-is-backdrop-filter-root.html"
}
```

## style[0]

```css

.background {
  background: cyan;
  width: 200px;
  height: 200px;
}
.shared {
  background: blue;
  position: relative;
  left: 50px;
  top: 50px;
  width: 100px;
  height: 100px;
  view-transition-name: shared;
  /* Apply transform to match the layerized pixels of view transition
     without establishing a backdrop filter root itself. */
  transform: translateZ(0);
}
.filter {
  backdrop-filter: invert(1);
  position: relative;
  left: -50px;
  top: 30px;
  width: 200px;
  height: 40px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
