# css/css-overflow/line-clamp/line-clamp-022.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-022.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 4;
  font: 16px / 32px serif;
  padding: 4px;
  background-color: yellow;
  border: 2px solid black;
}
.inner {
  background-color: purple;
  margin: 4px;
  /* Having a border means the margins of the .inner boxes won't collapse */
  border: 1px solid black;
}
.inner .inner {
  background-color: orange;
  white-space: pre;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
