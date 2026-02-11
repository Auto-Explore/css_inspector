# css/css-inline/initial-letter/initial-letter-raised-sunken-caps-sunken.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-raised-sunken-caps-sunken.html"
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

    .initial-letter::first-letter {
        initial-letter: 3 2;
        color: lime;
        font-size: 100px;   /* should be ignored in rendering */
        line-height: 50px;  /* should be ignored in rendering */
    }
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
