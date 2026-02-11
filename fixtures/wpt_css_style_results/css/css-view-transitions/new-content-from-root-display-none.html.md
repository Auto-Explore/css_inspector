# css/css-view-transitions/new-content-from-root-display-none.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-from-root-display-none.html"
}
```

## style[0]

```css

.tb { writing-mode: horizontal-tb; }
.lr { writing-mode: vertical-lr; }
.rl { writing-mode: vertical-rl; }
.shared {
  margin: 2px;
  width: 100px;
  height: 50px;
  background: green;
  contain: paint;
  border: 1px solid black;
}

html::view-transition-group(root) { animation-duration: 500s; }
html::view-transition-new(root) {
  animation: unset;
  opacity: 1;
}
html::view-transition-old(root) {
  animation: unset;
  opacity: 0;
}

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
