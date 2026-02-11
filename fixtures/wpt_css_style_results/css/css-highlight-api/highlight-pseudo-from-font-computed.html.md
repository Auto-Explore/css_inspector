# css/css-highlight-api/highlight-pseudo-from-font-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/highlight-pseudo-from-font-computed.html"
}
```

## style[0]

```css

  :root {
    font-size: 16px;
  }
  div {
    margin: 40px;
    font-size: 20px;
  }
  ::highlight(highlight1) {
    text-underline-offset: 0.5em;
    text-decoration-line: underline;
    text-decoration-color: green;
    text-decoration-thickness: 0.25rem;
  }
  #h2::highlight(highlight1) {
    text-underline-offset: 1.0em;
    text-decoration-line: underline;
    text-decoration-color: blue;
    text-decoration-thickness: 0.125rem;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
