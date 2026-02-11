# css/css-flexbox/gap-007-lr.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-007-lr.html"
}
```

## style[0]

```css

  body {
    writing-mode: vertical-lr;
  }

  section {
    background-color: green;
    block-size: 100px;
    inline-size: 200px;
    display: inline-flex;
    flex-direction: column;
    flex-wrap: wrap;
    gap: 20px;
    line-height: 18px;
  }
  section > div{
    background-color: grey;
    color: white;
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
