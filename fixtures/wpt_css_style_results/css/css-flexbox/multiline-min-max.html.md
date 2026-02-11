# css/css-flexbox/multiline-min-max.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/multiline-min-max.html"
}
```

## style[0]

```css

.flexbox {
  width: 600px;
  background-color: grey;
  border: 5px solid black;
  height: 20px;
  position: relative;
}
.flexbox > div {
  height: 10px;
}
.children-border-box > div {
  box-sizing: border-box;
}

.flexbox :nth-child(1) {
    background-color: blue;
}
.flexbox :nth-child(2) {
    background-color: yellow;
}
.flexbox :nth-child(3) {
    background-color: salmon;
}
.flexbox :nth-child(4) {
    background-color: lime;
}
.flexbox :nth-child(5) {
    background-color: red;
}
.flexbox :nth-child(6) {
    background-color: orange;
}
.flexbox :nth-child(7) {
    background-color: purple;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
