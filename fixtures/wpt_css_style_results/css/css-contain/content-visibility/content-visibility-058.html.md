# css/css-contain/content-visibility/content-visibility-058.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-058.html"
}
```

## style[0]

```css

.spacer {
  height: 10000px;
}
.container {
  width: 150px;
  background: lightblue;
  contain-intrinsic-size: 50px 250px;
}
.child {
  width: 50px;
  height: 300px;
  background: lightgreen;
}
#target {
  position: absolute;
  bottom: 0;

  width: 10px;
  height: 10px;
  background: blue;
}
.auto { content-visibility: auto; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
