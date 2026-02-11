# css/css-contain/contain-style-dynamic-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-dynamic-002.html"
}
```

## style[0]

```css

ol {
  list-style: inside decimal;
  padding-inline-start: 1em;
  counter-reset: c 0;
}
li {
  counter-increment: c 1;
}
div {
  contain: style;
}
:is(ol, li, div) {
  padding-left: 1em;
}
:is(ol, li, div)::before {
  content: "[::before=" counters(c, ".") "]";
}
:is(ol, li, div)::after {
  content: "[::after=" counters(c, ".") "]";
}
div::before {
  color: red;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
