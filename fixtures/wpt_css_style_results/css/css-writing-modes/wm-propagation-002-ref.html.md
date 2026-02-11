# css/css-writing-modes/wm-propagation-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-002-ref.html"
}
```

## style[0]

```css

html {
  direction: rtl;
}
body {
  height: 0;
}
html::before {
  content: "This text must be on the right";
}
html::after {
  content: "This text must be on the left";
  display: block;
  direction: ltr;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
