# css/css-ui/text-overflow-018.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/text-overflow-018.html"
}
```

## style[0]

```css

div{
  font-size: 25px;
  font-family: monospace;
  width: 2.1ch;
  white-space: pre;
  overflow: hidden;
  text-overflow: ellipsis;
  border: solid blue;
  padding-right: 0.9ch;
}
textarea {
  width: 100%;
  box-sizing: border-box;
  border: solid orange;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
