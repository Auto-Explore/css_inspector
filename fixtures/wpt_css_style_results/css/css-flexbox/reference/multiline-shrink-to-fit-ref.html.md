# css/css-flexbox/reference/multiline-shrink-to-fit-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/reference/multiline-shrink-to-fit-ref.html"
}
```

## style[0]

```css

.testcase {
    float: left;
    background-color: #aaa;
}

.testcase > :nth-child(1) {
    background-color: lightblue;
}
.testcase > :nth-child(2) {
    background-color: lightgreen;
}
.testcase > :nth-child(3) {
    background-color: pink;
}
.testcase > :nth-child(4) {
    background-color: yellow;
}

table {
  border-spacing: 0;
  border-collapse: collapse;
}

td {
  padding: 0;
  width: 100px;
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
