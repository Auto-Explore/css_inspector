# css/css-contain/contain-paint-005.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-005.html"
}
```

## style[0]

```css

rb {
  contain: paint;
  display: ruby-base;
  width: 0; /* Because if the test fails, this may get blockified, and which could make wide enough to hold the PASS */
}
rb::after {
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
