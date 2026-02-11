# css/css-variables/revert-rule-to-var.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/revert-rule-to-var.tentative.html"
}
```

## style[0]

```css

  #target {
    --green: green;
    color: var(--green);
  }
  #target {
    color: red;
    color: revert-rule;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
