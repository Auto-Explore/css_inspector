# css/css-view-transitions/fractional-box-with-shadow-new.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fractional-box-with-shadow-new.html"
}
```

## style[0]

```css

.box {
  box-shadow: -2px -3px 0 7px darkgreen;
  margin: 15px;
  background: green;
  width: 100.125px;
  height: 50.875px;
}
.shift {
  position: relative;
  left: 0.4px;
}
html::view-transition-group(*) { animation-duration: 300s; }
html::view-transition-new(*) { animation: unset; opacity: 1; }
html::view-transition-old(*) { animation: unset; opacity: 0; }
html::view-transition-group(root) { animation: unset; opacity: 0; }
html::view-transition { background: lightpink; }
```

```json
{
  "errors": 6,
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
