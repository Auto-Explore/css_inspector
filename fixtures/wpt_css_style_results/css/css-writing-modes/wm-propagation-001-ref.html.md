# css/css-writing-modes/wm-propagation-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-001-ref.html"
}
```

## style[0]

```css

html {
  writing-mode: vertical-rl;
}
body {
  writing-mode: vertical-rl;
  width: 0; height: 0;
}
html::before {
  content: "This text must be vertical.";
}
html::after {
  content: "This text must be horizontal.";
  display: block;
  writing-mode: horizontal-tb;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
