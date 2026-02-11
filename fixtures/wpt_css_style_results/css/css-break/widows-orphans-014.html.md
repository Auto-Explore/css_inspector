# css/css-break/widows-orphans-014.html

```json
{
  "format_version": 3,
  "file": "css/css-break/widows-orphans-014.html"
}
```

## style[0]

```css

  div
    {
      border: orange solid 4px;
      font-size: 20px;
      line-height: 1.3; /* computes to 26px */
      height: 234px; /* Therefore, exactly 9 line boxes */
      margin-bottom: 1em;
      padding: 0.5em; /* computes to 10px */
      width: 460px;

      columns: 4 auto;

      column-fill: auto;

      column-gap: 1em; /* computes to 20px */

      column-rule: blue solid 4px;
    }

  div#test
    {
      orphans: 7;
      widows: 3;
    }

  div#reference
    {
      orphans: 1;
      widows: 1;
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
