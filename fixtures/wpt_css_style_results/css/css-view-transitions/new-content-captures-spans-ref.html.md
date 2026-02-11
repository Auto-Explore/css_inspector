# css/css-view-transitions/new-content-captures-spans-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-captures-spans-ref.html"
}
```

## style[0]

```css

span {
  background: lightgreen;
  view-transition-name: span;
}
body {
  background: lightpink;
  font: 20px/1 Ahem;
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
      "message": "Unknown property “view-transition-name”.",
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
