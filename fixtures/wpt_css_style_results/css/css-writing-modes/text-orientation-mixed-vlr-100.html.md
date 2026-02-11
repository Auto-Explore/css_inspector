# css/css-writing-modes/text-orientation-mixed-vlr-100.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-mixed-vlr-100.html"
}
```

## style[0]

```css

@font-face {
    font-family: "orientation";
    src: url("/fonts/adobe-fonts/CSSHWOrientationTest.otf");
}
html {
    writing-mode: vertical-lr;
}
.test {
    font: 20px/1 "orientation";
    height: 17em;
    text-orientation: mixed;
    text-autospace: no-autospace;
  }
.line {
    white-space: pre;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
