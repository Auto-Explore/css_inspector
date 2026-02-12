# css/css-contain/content-visibility/content-visibility-animation-and-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-animation-and-scroll.html"
}
```

## style[0]

```css

#container {
  content-visibility: auto;
  contain-intrinsic-size: 100px 100px;
}
@keyframes unfade {
  from { opacity: 0; transform: none; }
  to { opacity: 1; transform: translate(100px); }
}
#target {
  background: green;
  height: 100px;
  width: 100px;
}
.animate {
  animation: unfade 1s linear 1 alternate;
  animation-fill-mode: forwards;
}
#spacer {
  height: 300vh;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
