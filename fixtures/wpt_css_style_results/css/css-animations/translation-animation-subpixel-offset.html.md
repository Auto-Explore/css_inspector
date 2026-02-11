# css/css-animations/translation-animation-subpixel-offset.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/translation-animation-subpixel-offset.html"
}
```

## style[0]

```css

@keyframes move {
  0% {transform: translateY(10px);}
  100% {transform: translateY(10px);}
}
#red {
  position: absolute;
  top: 11px;
  left: 1px;
  width: 100px;
  height: 100px;
  background: red;
}
#container {
  position: absolute;
  top: 0.4px;
  left: 0.6px;
}
#target {
  position: relative;
  top: 0.4px;
  left: 0.6px;
  width: 100px;
  height: 100px;
  background: green;
  animation: move 1s infinite alternate;
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
