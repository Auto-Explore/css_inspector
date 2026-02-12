# css/css-flexbox/gap-004-rtl-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-004-rtl-ref.html"
}
```

## style[0]

```css

  body {
    direction: rtl;
  }
  section {
    background-color: green;
    block-size: 100px;
    display: inline-flex;
  }
  section > div{
    background-color: grey;
    color: white;
  }
  section > div:not(:last-of-type) {
    margin-inline-end: 20px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
