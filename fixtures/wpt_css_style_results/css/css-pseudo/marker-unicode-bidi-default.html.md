# css/css-pseudo/marker-unicode-bidi-default.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-unicode-bidi-default.html"
}
```

## style[0]

```css

ol {
  float: left;
  list-style-position: inside;
}
.rtl {
  direction: rtl;
}

li:nth-child(1) { --marker: "\627 \644 " }
li:nth-child(2) { --marker: "\61 \627 \644 " }
li:nth-child(3) { --marker: "\627 \644 \62 " }
li:nth-child(4) { --marker: "\61 \627 \644 \62 " }
li:nth-child(5) { --marker: "\61 \62 \627 \644 " }
li:nth-child(6) { --marker: "\627 \644 \61 \62 " }
li:nth-child(7) { --marker: "\31 \627 \644 " }
li:nth-child(8) { --marker: "\627 \644 \32 " }
li:nth-child(9) { --marker: "\31 \627 \644 \32 " }

.string > li {
  list-style-type: var(--marker);
}
.marker > li::marker {
  content: var(--marker);
}
::marker {
  color: blue;
}
span {
  background: yellow;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
