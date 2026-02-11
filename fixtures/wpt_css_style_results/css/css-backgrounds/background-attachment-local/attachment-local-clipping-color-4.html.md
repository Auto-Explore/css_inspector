# css/css-backgrounds/background-attachment-local/attachment-local-clipping-color-4.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-attachment-local/attachment-local-clipping-color-4.html"
}
```

## style[0]

```css

#outer {
  width: 200px;
  height: 200px;
  padding: 40px;
  border: 10px double;
  overflow: hidden;
  border-radius: 50%;
  background: green local border-box;
}
#outer div {
  height: 500px;
}
p {
  margin-top: 20px;
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
