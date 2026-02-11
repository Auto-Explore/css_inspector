# css/css-flexbox/gap-008-ltr.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-008-ltr.html"
}
```

## style[0]

```css

  section {
    background-color: green;
    block-size: 100px;
    display: flex;
    gap: 40px 20px;
    flex-wrap: wrap;
  }
  section > div{
    background-color: grey;
    width: calc(50% - 10px);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
