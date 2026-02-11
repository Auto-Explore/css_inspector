# css/css-inline/initial-letter/Initial-letter-breaking-vlr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/Initial-letter-breaking-vlr-ref.html"
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

    .mc {
        columns: 2;
        direction: ltr;
        height: 500px;
        writing-mode: vertical-lr;
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
