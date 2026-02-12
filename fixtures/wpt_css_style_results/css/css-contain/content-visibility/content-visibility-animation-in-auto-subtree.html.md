# css/css-contain/content-visibility/content-visibility-animation-in-auto-subtree.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-animation-in-auto-subtree.html"
}
```

## style[0]

```css

#container {
  content-visibility: auto;
}
@keyframes fade {
  from { opacity: 1; }
  to { opacity: 0;  }
}
#target {
  background: green;
  height: 100px;
  width: 100px;
}
.animate {
  animation: fade 1s linear 2 alternate;
}
.transition {
  transition: opacity 1s linear;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
