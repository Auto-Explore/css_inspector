# css/css-contain/contain-paint-006.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-006.html"
}
```

## style[0]

```css

rbc {
  contain: paint;
  display: ruby-base-container;
  width: 0; /* Because if the test fails, this may get blockified, and which could make wide enough to hold the PASS */
}
rbc::after {
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
