# css/css-flexbox/align-self-016.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-self-016.html"
}
```

## style[0]

```css

#container {
  display: flex;
  flex-flow: column wrap;
  width: 100px;
  border-right: 100px solid red;
}
#item{
  align-self: start;
  background: linear-gradient(to bottom, red 50%, green 50%);
}
.float {
  float: left;
  width: 100px;
  height: 100px;
  background: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
