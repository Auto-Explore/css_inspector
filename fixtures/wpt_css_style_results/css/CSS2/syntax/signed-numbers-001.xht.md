# css/CSS2/syntax/signed-numbers-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/signed-numbers-001.xht"
}
```

## style[0]

```css

  div {background: red; width: 20px; height: 100px}
  .s1 {background: green; height: 20px}
  .s2 {background: green; height: 10px; height: +20px}
  .s3 {background: green; height: 20px; height: 1 0px}
  .s4 {background: green; height: 20px; height: 1/*comment*/0px}
  .s5 {background: green; height: 20px; height: + 10px}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “height”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “height”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “height”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
