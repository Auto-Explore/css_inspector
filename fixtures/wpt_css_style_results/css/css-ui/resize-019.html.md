# css/css-ui/resize-019.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/resize-019.html"
}
```

## style[0]

```css

#test {
  position: absolute;
  background: orange;
  height: 100px;
  width: 100px;
  overflow: auto;
  resize: both;
}
#ref {
  position: absolute;
  background: green;
  height: 100px;
  width: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
