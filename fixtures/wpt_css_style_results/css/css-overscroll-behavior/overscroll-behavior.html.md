# css/css-overscroll-behavior/overscroll-behavior.html

```json
{
  "format_version": 3,
  "file": "css/css-overscroll-behavior/overscroll-behavior.html"
}
```

## style[0]

```css

.outer {
  height: 400px;
  width: 1000px;
  background: white
}
.content {
  height: 600px;
  width: 1200px;
}
#root {
  overflow: scroll;
  height: 600px;
  width: 800px;
  background: white;
}
#container {
  overflow: scroll;
}
#non_scrollable {
  overflow: clip;
}
#green {
  background: repeating-linear-gradient(to bottom right, green 15%, white 30%);
}
#blue {
  background: repeating-linear-gradient(to bottom right, blue 15%, white 30%);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
