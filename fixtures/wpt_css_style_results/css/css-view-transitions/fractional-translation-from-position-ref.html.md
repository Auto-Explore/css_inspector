# css/css-view-transitions/fractional-translation-from-position-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fractional-translation-from-position-ref.html"
}
```

## style[0]

```css

body {
  width: 100vw;
  height: 100vh;
  background: grey;
  font: 12px/1 Ahem;
}

#target {
  width: 100px;
  height: 100px;
  position: fixed;
  top: 100.52px;
  left: 100.37px;

  view-transition-name: target;
  contain: layout;
  will-change: filter;
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
