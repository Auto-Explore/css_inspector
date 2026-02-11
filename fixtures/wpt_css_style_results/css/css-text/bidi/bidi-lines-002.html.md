# css/css-text/bidi/bidi-lines-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/bidi/bidi-lines-002.html"
}
```

## style[0]

```css

div {
    direction: rtl;
    unicode-bidi: plaintext;
    text-align: start;
    font-size: 2em;
    width: 10em;
    border: solid; padding: 0 0.5ch;
}
.l { color: blue; }
.r { color: orange; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
