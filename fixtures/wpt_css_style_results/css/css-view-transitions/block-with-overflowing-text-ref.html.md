# css/css-view-transitions/block-with-overflowing-text-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/block-with-overflowing-text-ref.html"
}
```

## style[0]

```css

:root {
  font: 20px/1 Ahem;
  scrollbar-width: none;
}

#target {
  text-shadow: red -20px -50px;
  position: relative;
  top: 100px;
  left: 100px;
}

body {
  background: pink;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
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
