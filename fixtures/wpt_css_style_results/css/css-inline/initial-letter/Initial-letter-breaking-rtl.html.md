# css/css-inline/initial-letter/Initial-letter-breaking-rtl.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/Initial-letter-breaking-rtl.html"
}
```

## style[0]

```css

    .sample {
        direction: rtl;
        font-family: Ahem;
        font-size: 20px;
        line-height: 24px;
        width: 230px;
    }

    .float {
        background: cyan;
        clear: none;
        float: right;
        height: 50px;
        width: 50px;
    }

    .mc {
        columns: 2;
        direction: rtl;
        height: 500px;
        writing-mode: horizontal-tb;
    }

    .initial-letter::first-letter {
        initial-letter: 3 drop;
        color: lime;
        font-size: 100px;   /* should be ignored in rendering */
        line-height: 50px;  /* should be ignored in rendering */
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “initial-letter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
