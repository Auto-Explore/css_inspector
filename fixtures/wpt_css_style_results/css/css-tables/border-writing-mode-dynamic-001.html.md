# css/css-tables/border-writing-mode-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/border-writing-mode-dynamic-001.html"
}
```

## style[0]

```css

  .bordermix {
    border-top: 10px solid red;
    border-bottom: 20px solid green;
    border-left: 30px solid yellow;
    border-right: 40px solid orange;
  }

  .vertical {
    writing-mode: vertical-rl;
  }

  main td {
    width:100px;
    height:100px;
  }
  main table {
    border-collapse: collapse;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
