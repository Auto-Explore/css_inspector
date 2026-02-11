# css/css-values/attr-revert-rule.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-revert-rule.tentative.html"
}
```

## style[0]

```css

  :root {
    #test1 {
      color: green;
    }
    #test1 {
      color: red;
      color: attr(data-unknown, revert-rule);
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
