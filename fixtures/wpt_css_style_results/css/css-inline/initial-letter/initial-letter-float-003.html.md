# css/css-inline/initial-letter/initial-letter-float-003.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-float-003.html"
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
