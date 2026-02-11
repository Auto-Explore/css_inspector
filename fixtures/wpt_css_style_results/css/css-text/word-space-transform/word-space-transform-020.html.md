# css/css-text/word-space-transform/word-space-transform-020.html

```json
{
  "format_version": 3,
  "file": "css/css-text/word-space-transform/word-space-transform-020.html"
}
```

## style[0]

```css

div {
  font-size: 2em;
  border: solid black;
  margin: 5px;
  width: 30ch;
}
#test {
  word-space-transform: none;
}
#test span {
  word-space-transform: ideographic-space auto-phrase;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “word-space-transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
