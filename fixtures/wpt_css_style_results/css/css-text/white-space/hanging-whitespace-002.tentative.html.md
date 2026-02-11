# css/css-text/white-space/hanging-whitespace-002.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/hanging-whitespace-002.tentative.html"
}
```

## style[0]

```css

  div {
    font: 25px/1 Ahem;
    width: 5ch;
    height: 4ch;
    margin-left: -1ch;
    background:
      linear-gradient(red, red) 4ch 0 / 1ch 4ch no-repeat,
      linear-gradient(green, green) 1ch 0 / 3ch 4ch no-repeat;

    white-space: normal;
    text-align: right;
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
