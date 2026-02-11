# css/css-contain/contain-size-monolithic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-monolithic-001.html"
}
```

## style[0]

```css

  div#multi-column
    {
      column-count: 2;
      column-fill: balance; /* balance is the initial column-fill value */
      column-gap: 4ch;
      column-rule: red solid 4ch;
      column-width: 2ch;
      font-family: monospace;
      font-size: 20px;
      width: 8ch;
    }

  div#size-contain
    {
      contain: size;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
