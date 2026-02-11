# css/selectors/invalidation/has-invalidation-first-in-sibling-chain.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-invalidation-first-in-sibling-chain.html"
}
```

## style[0]

```css

div, main { color: grey }
#subject1:has(.item ~ .item + .item) { color: red; }
#subject2:has(.item + .item + .item) { color: orangered; }
#subject3:has(.item .item ~ .item + .item) { color: green; }
#subject4:has(.item .item + .item + .item) { color: lightgreen; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
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
