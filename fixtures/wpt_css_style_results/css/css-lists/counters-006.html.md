# css/css-lists/counters-006.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counters-006.html"
}
```

## style[0]

```css

  * {
    padding: 0;
    margin: 0;
  }

  .test {
    counter-increment: section;
    counter-reset: multilevel;
  }

  ol[multilevel] > li::before {
    content: counter(section) "." counters(multilevel, ".") ".";
    counter-increment: multilevel;
  }

  ol[multilevel] {
    list-style: none !important;
    clear: both;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
