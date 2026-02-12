# css/css-contain/content-visibility/content-visibility-061.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-061.html"
}
```

## style[0]

```css

.spacer {
  width: 150px;
  height: 3000px;
  background: lightblue;
}
#container {
  width: 150px;
  height: 150px;
  background: red;
}
#target {
  display: contents;

  position: relative;
  top: 75px;

  width: 50px;
  height: 50px;
  background: pink;
}
.hidden {
  content-visibility: hidden;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
