# css/css-contain/contain-paint-007.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-007.html"
}
```

## style[0]

```css

rtc {
  contain: paint;
  display: ruby-text-container;
  width: 0; /* Because if the test fails, this may get blockified, and which could make wide enough to hold the PASS */
  font-size: 1rem;
}
rtc::after {
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
