# css/css-inline/initial-letter/initial-letter-with-tab-rtl.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-with-tab-rtl.html"
}
```

## style[0]

```css

    .sample {
        direction: rtl;
        font-family: Ahem;
        font-size: 20px;
        line-height: 24px;
    }

    .initial-letter::first-letter {
        initial-letter: 3 drop;
        color: lime;
        font-size: 100px;   /* should be ignored in rendering */
        line-height: 50px;  /* should be ignored in rendering */
    }

    .sample { white-space: pre; }
    .initial-letter::first-letter { background: yellow; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
