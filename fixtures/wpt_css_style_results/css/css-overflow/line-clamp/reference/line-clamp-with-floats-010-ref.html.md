# css/css-overflow/line-clamp/reference/line-clamp-with-floats-010-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/reference/line-clamp-with-floats-010-ref.html"
}
```

## style[0]

```css

#scrollContainer {
  overflow: scroll;
  font: 16px / 32px serif;
  height: 4lh;
  border: 1px solid black;
}
.clamp {
  display: flow-root;
  padding: 0 4px;
  white-space: pre;
  background-color: yellow;
}
.float {
  float: left;
  width: 50px;
  height: 50px;
  margin: 4px;
  background-color: skyblue;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
