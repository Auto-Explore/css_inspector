# css/css-flexbox/gap-007-ltr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-007-ltr-ref.html"
}
```

## style[0]

```css

  section {
    background-color: green;
    block-size: 100px;
    inline-size: 200px;
    display: inline-flex;
    flex-direction: column;
    flex-wrap: wrap;
    line-height: 18px;
  }
  section > div{
    background-color: grey;
    color: white;
  }
  section > div:nth-child(2) {
    margin-block: 20px;
  }
  section > div:not(:last-of-type) {
    margin-inline-end: 20px;
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
