# css/css-flexbox/gap-002-ltr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-002-ltr-ref.html"
}
```

## style[0]

```css

  section {
    background-color: green;
    block-size: 200px;
    display: flex;
    flex-direction: column;
  }
  section > div{
    background-color: grey;
    flex: 1 1 auto;
  }
  section > div:not(:first-child) {
    margin-block-start: 20px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
