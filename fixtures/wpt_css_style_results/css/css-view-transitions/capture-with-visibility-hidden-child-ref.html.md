# css/css-view-transitions/capture-with-visibility-hidden-child-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/capture-with-visibility-hidden-child-ref.html"
}
```

## style[0]

```css

body {
  background: pink;
}
.target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
  background: blue;
}
.invisible {
  width: 500px;
  height: 500px;
  visibility: hidden;
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
