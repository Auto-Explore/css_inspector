# css/css-text/overflow-wrap/overflow-wrap-break-word-008.html

```json
{
  "format_version": 3,
  "file": "css/css-text/overflow-wrap/overflow-wrap-break-word-008.html"
}
```

## style[0]

```css

div {
  position: relative;
  font-family: Ahem;
  font-size: 25px;
  line-height: 1em;
}
.expected {
  position: absolute;
  color: green;
  width: 100px;
  height: 100px;
  white-space: pre;
}
.test {
  background: green;
  color: red;
  width: 4ch;
  z-index: -1;

  white-space: break-spaces;
  overflow-wrap: break-word;
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
