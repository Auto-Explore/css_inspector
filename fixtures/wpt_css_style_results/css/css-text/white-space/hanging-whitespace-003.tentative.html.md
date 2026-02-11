# css/css-text/white-space/hanging-whitespace-003.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/hanging-whitespace-003.tentative.html"
}
```

## style[0]

```css

  div {
    font: 25px/1 Ahem;
    width: 4ch;
    height: 4ch;
    background:
      linear-gradient(green, green) 0 1ch / 1ch 1ch no-repeat,
      linear-gradient(green, green) 0 2ch / 2ch 1ch no-repeat,
      linear-gradient(green, green) 1ch 3ch / 3ch 1ch no-repeat,
      red;

    text-align: right;
    white-space: normal;
    color: green;
  }
  .pre-wrap {
    white-space: pre-wrap;
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
