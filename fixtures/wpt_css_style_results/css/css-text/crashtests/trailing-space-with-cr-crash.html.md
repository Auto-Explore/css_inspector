# css/css-text/crashtests/trailing-space-with-cr-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-text/crashtests/trailing-space-with-cr-crash.html"
}
```

## style[0]

```css

div {
  width: 100px;
  white-space: pre-wrap;
  word-break: break-word;
  border: 1px solid blue;
}
.atomic {
  display: inline-block;
  width: 99px;
  height: 1em;
  background: orange;
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
