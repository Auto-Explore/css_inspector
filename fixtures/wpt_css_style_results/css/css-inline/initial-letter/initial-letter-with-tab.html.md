# css/css-inline/initial-letter/initial-letter-with-tab.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-with-tab.html"
}
```

## style[0]

```css

    .sample {
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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “initial-letter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
