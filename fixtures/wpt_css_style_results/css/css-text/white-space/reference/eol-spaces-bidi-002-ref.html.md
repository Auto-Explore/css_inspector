# css/css-text/white-space/reference/eol-spaces-bidi-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/reference/eol-spaces-bidi-002-ref.html"
}
```

## style[0]

```css

@font-face {
    font-family: "Hasubi Mono";
    src: url("/fonts/hasubi-mono/HasubiMono-Regular.woff2");
}

div {
    font-family: "Hasubi Mono", monospace;
    border: solid blue;
    font-size: 1.5em;
    white-space: pre;
}
.ref {
    border-color: orange;
}
.w6 { width: 6.01ch; }
.w7 { width: 7.01ch; }
.w8 { width: 8.01ch; }
.w9 { width: 9.01ch; }

.blue { background: #aaaaff; }
.red { background: #ffaaaa; }
.green { background: #aaffaa; }
.pink { background: #ffaaff; }
.yellow { background: #ffffaa; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
