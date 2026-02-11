# css/css-inline/initial-letter/initial-letter-float-001-vlr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-float-001-vlr-ref.html"
}
```

## style[0]

```css

    .sample {
        border: solid 1px green;
        font-family: Ahem;
        font-size: 20px;
        height: 230px;
        line-height: 24px;
        writing-mode: vertical-lr;
    }

    .float {
        background: cyan;
        clear: none;
        float: left;
        height: 50px;
        width: 50px;
    }

    .fake-initial-letter {
        background: lime;
        float: left;
        height: 80px;
        margin-left: -4px;
        width: 80px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
