# css/css-animations/display-none-to-display-block-dont-cancel.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/display-none-to-display-block-dont-cancel.tentative.html"
}
```

## style[0]

```css

@keyframes display-animation {
  0% { display: none; }
  100% { display: block; }
}
#target {
  display: none;
}
#target.animate {
  animation: display-animation 1s;
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
