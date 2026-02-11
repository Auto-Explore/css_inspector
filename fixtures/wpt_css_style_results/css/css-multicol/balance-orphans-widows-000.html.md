# css/css-multicol/balance-orphans-widows-000.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/balance-orphans-widows-000.html"
}
```

## style[0]

```css

  .multicol {
      columns: 2;
      orphans: 4;
      widows: 4;
      width: 100px;
      column-gap: 80px;
      column-rule: 80px solid green;
      line-height: 0;
      background: red;
  }
  span {
      display: inline-block;
      width: 100%;
      height: 10px;
      background: green;
  }
  span.tall {
      height: 40px;
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
