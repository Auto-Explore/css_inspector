# css/css-flexbox/gap-009-ltr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-009-ltr-ref.html"
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
  section > div{
    background-color: grey;
  }
  section > div {
    width: calc(50% - .5rem);
  }
  section > div:nth-child(1),
  section > div:nth-child(3) {
    margin-inline-end: 1rem;
  }
  section > div:nth-child(3),
  section > div:nth-child(4) {
    margin-block-start: 50px; /* half block size */
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
