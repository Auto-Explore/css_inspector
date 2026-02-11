# css/css-text/word-space-transform/word-space-transform-015-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-text/word-space-transform/word-space-transform-015-manual.html"
}
```

## style[0]

```css

div, textarea {
  font-size: 2em;
  margin: 1em;
  font-family: monospace;
  resize: none;
  overflow: hidden;
  height: 2em;
  width: 20ch;
  padding: 0px;
}
#test {
  border: solid blue;
  word-space-transform: space;
}
#ref {
  border: solid orange;
}
textarea {
  border: solid pink;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
