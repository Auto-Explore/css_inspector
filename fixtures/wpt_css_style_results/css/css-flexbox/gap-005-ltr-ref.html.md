# css/css-flexbox/gap-005-ltr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-005-ltr-ref.html"
}
```

## style[0]

```css

  section {
    background-color: green;
    display: inline-flex;
    flex-direction: column;
  }
  section > div{
    background-color: grey;
    color: white;
  }
  section > div:not(:last-of-type) {
    margin-block-end: 20px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
