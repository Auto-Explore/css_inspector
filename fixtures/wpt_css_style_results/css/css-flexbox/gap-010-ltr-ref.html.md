# css/css-flexbox/gap-010-ltr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-010-ltr-ref.html"
}
```

## style[0]

```css

  section {
    background-color: green;
    block-size: 100px;
    display: flex;
    flex-wrap: wrap;
  }
  section > div {
    background-color: grey;
    flex: 1 1 auto;
  }
  section > div:not(:first-child) {
    margin-inline-start: calc(10% - 1rem / 2);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
