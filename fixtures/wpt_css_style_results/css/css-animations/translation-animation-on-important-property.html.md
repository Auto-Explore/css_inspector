# css/css-animations/translation-animation-on-important-property.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/translation-animation-on-important-property.html"
}
```

## style[0]

```css

@keyframes move {
  0% {transform: translateX(0px);}
  100% {transform: translateX(100px);}
}
#target {
  width: 100px;
  height: 100px;
  background: green;
  transform:  none !important;
  animation: move 10000s cubic-bezier(0, 1, 1, 0) -5000s;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
