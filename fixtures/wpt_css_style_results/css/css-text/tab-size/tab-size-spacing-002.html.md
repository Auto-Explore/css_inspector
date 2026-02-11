# css/css-text/tab-size/tab-size-spacing-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/tab-size/tab-size-spacing-002.html"
}
```

## style[0]

```css

div {
  font-family: monospace; /* so we can compare tab with a count of preserved spaces */
  white-space: pre;
}
.test {
  letter-spacing: 1ch; /* effectively double the advance of the characters */
  tab-size: 3;
}
.ref1 {
  letter-spacing: 1ch;
}
.ref2 {
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
