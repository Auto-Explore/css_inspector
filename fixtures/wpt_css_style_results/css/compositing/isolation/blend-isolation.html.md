# css/compositing/isolation/blend-isolation.html

```json
{
  "format_version": 3,
  "file": "css/compositing/isolation/blend-isolation.html"
}
```

## style[0]

```css

.a {
  background-color: rgb(0,255,0);
}
#b {
  width: 200px;
  height: 210px;
}
.c {
  width: 100px;
  height: 100px;
  mix-blend-mode: difference;
}
#d {
  isolation: isolate;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
