# css/css-contain/content-visibility/content-visibility-animation-becomes-visible.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-animation-becomes-visible.html"
}
```

## style[0]

```css

#container {
  content-visibility: hidden;
}
@keyframes fade {
  from { opacity: 0; transform: none; }
  to { opacity: 1; transform: translate(100px); }
}
#target {
  background: green;
  height: 100px;
  width: 100px;
}
.animate {
  animation: fade 1s linear 1 alternate;
  animation-fill-mode: forwards;
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
