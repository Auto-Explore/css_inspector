# css/css-view-transitions/fragmented-during-transition-skips.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fragmented-during-transition-skips.html"
}
```

## style[0]

```css

#spacer {
  width: 100px;
  height: 950px;
  background: lightgreen;
}
#container {
  width: 500px;
  height: 500px;
}
.fragment {
  columns: 2;
}
#target {
  width: 200px;
  height: 200px;
  background: green;
  view-transition-name: target;
}
#unrelated {
  width: 100px;
  height: 100px;
  background: lightblue;
  view-transition-name: unrelated;
}

::view-transition {
  background: pink;
}
::view-transition-group(root) {
  animation-duration: 500s;
  visibility: hidden;
}
::view-transition-group(target) {
  border: 1px solid black;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
