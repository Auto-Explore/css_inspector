# css/css-break/reference/widows-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/reference/widows-001-ref.html"
}
```

## style[0]

```css

  div
    {
      border: orange solid 4px;
      font-size: 20px;
      line-height: 1.3; /* computes to 26px */
      height: 104px; /* Therefore, exactly 4 line boxes */
      margin-bottom: 1em;
      padding: 0.5em; /* computes to 10px */
      width: 490px;

      columns: 3 auto;
      column-fill: auto;
      column-gap: 1em; /* computes to 20px */
      column-rule: blue solid 4px;

      widows: 1;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
