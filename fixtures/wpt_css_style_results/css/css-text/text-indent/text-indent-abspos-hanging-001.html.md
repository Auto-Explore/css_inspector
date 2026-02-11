# css/css-text/text-indent/text-indent-abspos-hanging-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-indent/text-indent-abspos-hanging-001.html"
}
```

## style[0]

```css

.test {
  line-height: 1;
  width: 8ch;
}
.hanging {
  text-indent: 2ch hanging;
}
.marker {
  display: inline;
  position: absolute;
  width: 4ch;
  height: 1em;
  background: lime;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-indent”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
