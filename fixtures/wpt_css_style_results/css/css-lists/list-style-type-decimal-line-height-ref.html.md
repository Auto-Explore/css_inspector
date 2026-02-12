# css/css-lists/list-style-type-decimal-line-height-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/list-style-type-decimal-line-height-ref.html"
}
```

## style[0]

```css

  div {
    display: grid;
    grid-template-columns: 100px 100px auto;
    align-items: start;
    background: red;
  }
  li {
    line-height: 100%;
    counter-set: list-item 1;
  }
  ol {
    margin: 0;
    padding: 0;
    padding-left: 40px;
    background: green;
  }
  ::marker {
    content: "1. ";
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
