# css/css-view-transitions/scoped/scroller-with-child.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/scroller-with-child.html"
}
```

## style[0]

```css


body { margin: 20px; font: 18pt monospace; line-height: 30px; }
#scope { overflow: auto; contain: layout; padding: 20px;
  width: 200px; height: 100px; border: 20px solid #acf; }
#part { view-transition-name: foo;
  background: #fea; padding: 10px; border: 4px solid orange;
  width: 150px; height: 180px; margin: 20px 0 0 90px; }
#scope::view-transition-group(*) { animation-play-state: paused; }
#scope::view-transition-new(*) { animation: unset; opacity: 1; }
#scope::view-transition-old(*) { animation: unset; opacity: 0; }

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
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
