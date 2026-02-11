# css/css-contain/contain-paint-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-002.html"
}
```

## style[0]

```css

span {
  contain: paint;
  width: 0; /* Because if the test fails, the span may get blockified, and which would make wide enough to hold the PASS */
}
span::after {
  content: "PASS";
  position: absolute;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
