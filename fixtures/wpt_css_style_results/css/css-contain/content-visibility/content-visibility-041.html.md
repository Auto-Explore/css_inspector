# css/css-contain/content-visibility/content-visibility-041.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-041.html"
}
```

## style[0]

```css

#container {
  width: 150px;
  height: 150px;
  background: lightblue;
}
#container::before {
  content: "FAIL! ";
  color: red;
}
.hasAfter::after {
  content: "FAIL!";
  background: red;
  color: white;
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
