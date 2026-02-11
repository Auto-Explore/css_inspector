# css/css-view-transitions/root-to-shared-animation-incoming.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/root-to-shared-animation-incoming.html"
}
```

## style[0]

```css

body {
  background: lightgreen;
  padding: 0;
  margin: 0;
}
#box {
  width: 100px;
  height: 120px;
  background: blue;
  contain: paint;
}

html::view-transition-group(*) { animation-delay: 300s; }
html::view-transition-new(*) { animation: unset; opacity: 1; }
html::view-transition-old(*) { animation: unset; opacity: 0; }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
    }
  ],
  "warnings": 0
}
```
