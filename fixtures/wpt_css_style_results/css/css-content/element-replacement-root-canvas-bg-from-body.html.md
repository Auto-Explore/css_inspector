# css/css-content/element-replacement-root-canvas-bg-from-body.html

```json
{
  "format_version": 3,
  "file": "css/css-content/element-replacement-root-canvas-bg-from-body.html"
}
```

## style[0]

```css

:root {
  content: url('resources/rect.svg');
}
body {
  background-color: aquamarine;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
