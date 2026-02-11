# css/css-flexbox/percentage-heights-009.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/percentage-heights-009.html"
}
```

## style[0]

```css

.container {
  height: 100px;
  width: 100px;
}
.flexbox {
  background: red;
  display: flex;
  flex-direction: column;
  height: 100%;
}
.first-item {
  background: green;
  display: flex;
}
.second-item {
  /* This should not be considered indefinite */
  height: 100%;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
