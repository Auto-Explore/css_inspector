# css/css-inline/initial-letter/initial-letter-with-first-line.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-with-first-line.html"
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

    .sample::first-line { color: cyan; }
    .initial-letter::first-letter {
        initial-letter: 3 drop;
        color: inherit;
        font-size: 100px;   /* should be ignored in rendering */
        line-height: 50px;  /* should be ignored in rendering */
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
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
