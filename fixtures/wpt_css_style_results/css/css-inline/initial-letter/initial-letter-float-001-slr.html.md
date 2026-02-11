# css/css-inline/initial-letter/initial-letter-float-001-slr.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-float-001-slr.html"
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
        writing-mode: sideways-lr;
    }

    .float {
        background: cyan;
        clear: none;
        float: left;
        height: 50px;
        width: 50px;
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
