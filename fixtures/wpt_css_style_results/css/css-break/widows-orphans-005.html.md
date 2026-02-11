# css/css-break/widows-orphans-005.html

```json
{
  "format_version": 3,
  "file": "css/css-break/widows-orphans-005.html"
}
```

## style[0]

```css

  .multicol {
      position: relative;
      columns: 3;
      column-fill: auto;
      column-gap: 10px;
      width: 320px;
      height: 200px;
      orphans: 1;
      widows: 2;
      column-rule: 1px dotted;
      line-height: 20px;
  }
  .ibk {
      display: inline-block;
      width: 70px;
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
