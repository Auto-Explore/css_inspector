# css/css-flexbox/multiline-reverse-wrap-baseline.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/multiline-reverse-wrap-baseline.html"
}
```

## style[0]

```css

.flexbox {
  width: 200px;
  display: flex;
  background-color: #aaa;
  position: relative;
  flex-wrap: wrap-reverse;
  align-items: baseline;
  margin-bottom: 10px;
}
.flexbox > div {
  border: 0;
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
