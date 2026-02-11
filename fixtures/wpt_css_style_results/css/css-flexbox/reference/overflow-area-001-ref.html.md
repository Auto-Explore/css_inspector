# css/css-flexbox/reference/overflow-area-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/reference/overflow-area-001-ref.html"
}
```

## style[0]

```css


.set {
  clear: both;
  margin: 1em;
}

.ref {
  display: flex;
  overflow: scroll;
  border: solid blue;
  background: aqua;
  margin: 0.5em;
  float: left;
  width: 80px;
  height: 80px;
}
div {
  flex: none;
}

.ref:first-child {
  border-color: orange;
}
.ref > div {
  border: 30px solid aqua;
  width: 30px;
  height: 30px;
  background: teal;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
