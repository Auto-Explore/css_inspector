# css/css-flexbox/multiline-shrink-to-fit.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/multiline-shrink-to-fit.html"
}
```

## style[0]

```css

.flexbox {
  display: flex;
  background-color: #aaa;
  position: relative;
  flex-wrap: wrap;
  flex-direction: column;
  float: left;
  align-content: flex-start;
}
.flexbox > * {
  flex: none;
}
.flexbox :nth-child(1) {
  background-color: lightblue;
}
.flexbox :nth-child(2) {
  background-color: lightgreen;
}
.flexbox :nth-child(3) {
  background-color: pink;
}
.flexbox :nth-child(4) {
  background-color: yellow;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
