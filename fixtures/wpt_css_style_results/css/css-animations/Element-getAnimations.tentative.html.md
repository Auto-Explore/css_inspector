# css/css-animations/Element-getAnimations.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/Element-getAnimations.tentative.html"
}
```

## style[0]

```css

@keyframes anim1 {
  to { left: 100px }
}
@keyframes anim2 {
  to { top: 100px }
}
@keyframes multiPropAnim {
  to { background: green, opacity: 0.5, left: 100px, top: 100px }
}
::before {
  content: ''
}
::after {
  content: ''
}
@keyframes empty { }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Missing ';' between declarations.",
      "severity": "Error"
    },
    {
      "message": "Missing ';' between declarations.",
      "severity": "Error"
    },
    {
      "message": "Missing ';' between declarations.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
