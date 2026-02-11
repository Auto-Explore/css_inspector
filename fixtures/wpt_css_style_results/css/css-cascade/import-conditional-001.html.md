# css/css-cascade/import-conditional-001.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/import-conditional-001.html"
}
```

## style[0]

```css

  @import "support/test-red.css";
  @import "support/test-green.css"
    (min-width: 1px) and /* assuming screen < 1km */ (max-width: 40000in), nonsense;
  @import "support/test-red.css"
    (max-width: 1px), nonsense;
  div {
    box-sizing: border-box;
    width: 100px;
    height: 100px;
    padding: 5px; /* Avoids text antialiasing issues */
    background: red;
  }
  
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```
