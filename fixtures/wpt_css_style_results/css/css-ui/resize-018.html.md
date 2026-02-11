# css/css-ui/resize-018.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/resize-018.html"
}
```

## style[0]

```css

#test {
  position: absolute;
  width: 100px;
  height: 100px;
  background: orange;
  overflow: hidden;
  resize: both;
}
#target {
  position: absolute;
  width: 150px;
  height: 150px;
  background: blue;
}
#log { margin-top: 200px; }
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
