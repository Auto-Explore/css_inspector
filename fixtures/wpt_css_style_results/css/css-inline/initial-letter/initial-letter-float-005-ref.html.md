# css/css-inline/initial-letter/initial-letter-float-005-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-float-005-ref.html"
}
```

## style[0]

```css

    .sample {
        border: solid 1px green;
        font-family: Ahem;
        font-size: 20px;
        line-height: 24px;
        width: 230px;
    }

    .float {
        background: cyan;
        clear: left;
        float: right;
        height: 50px;
        width: 50px;
    }

    .fake-initial-letter {
        background: lime;
        float: left;
        height: 80px;
        margin-top: 2px;
        width: 80px;
    }
    .float { margin-top: -58px; }
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
