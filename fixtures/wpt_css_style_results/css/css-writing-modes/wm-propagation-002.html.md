# css/css-writing-modes/wm-propagation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-002.html"
}
```

## style[0]

```css

body {
  direction: rtl;
  height: 0;
}
html::before {
  content: "This text must be on the right";
}
html::after {
  content: "This text must be on the left";
  display: block;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
