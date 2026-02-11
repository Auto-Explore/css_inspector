# css/css-multicol/column-fill-balance-orthog-block-001.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/column-fill-balance-orthog-block-001.html"
}
```

## style[0]

```css

  div#multi-col
    {
      background-color: yellow; /* Not part of the test */
      columns: 2 auto;
      column-fill: balance; /* Balance content equally between columns, if possible. */
      height: 250px; /* more than enough to display "TEXT" */
    }

  div#unbreakable-block
    {
      background-color: lime; /* Not part of the test */
      font-size: 50px;
      line-height: 1.2; /* Not part of the test */
      writing-mode: vertical-rl;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
