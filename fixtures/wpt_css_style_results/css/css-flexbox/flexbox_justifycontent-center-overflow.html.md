# css/css-flexbox/flexbox_justifycontent-center-overflow.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_justifycontent-center-overflow.html"
}
```

## style[0]

```css

.flexbox {
  background: blue;
  margin-left: 50px;
  height: 150px;
  width: 50px;
  display: flex;
  justify-content: center;
  position: relative;
}
span {
  background: orange;
  margin: 10px;
  flex: 1 0 40px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
