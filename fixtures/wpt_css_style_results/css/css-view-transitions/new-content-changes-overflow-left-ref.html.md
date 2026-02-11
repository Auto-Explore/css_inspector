# css/css-view-transitions/new-content-changes-overflow-left-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-changes-overflow-left-ref.html"
}
```

## style[0]

```css

body { background: pink }
#target {
  position: relative;
  background: green;
  left: 10px;
  width: 100px;
  height: 100px;
  view-transition-name: target;
}
#target.toggle {
  outline: 300px solid transparent;
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
