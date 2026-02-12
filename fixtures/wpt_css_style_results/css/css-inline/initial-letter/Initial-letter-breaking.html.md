# css/css-inline/initial-letter/Initial-letter-breaking.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/Initial-letter-breaking.html"
}
```

## style[0]

```css

    .sample {
        font-family: Ahem;
        font-size: 20px;
        line-height: 24px;
        width: 230px;
    }

    .float {
        background: cyan;
        clear: none;
        float: left;
        height: 50px;
        width: 50px;
    }

    .mc { columns: 2;}

    .initial-letter::first-letter {
        initial-letter: 3 drop;
        color: lime;
        font-size: 100px;   /* should be ignored in rendering */
        line-height: 50px;  /* should be ignored in rendering */
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
