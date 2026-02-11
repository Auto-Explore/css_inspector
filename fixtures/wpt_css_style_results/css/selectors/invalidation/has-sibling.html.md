# css/selectors/invalidation/has-sibling.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-sibling.html"
}
```

## style[0]

```css

div, main { color: grey }
#subject:has(~ .test) { color: red }
#subject:has(+ .test) { color: green }
#subject:has(~ div .test) { color: blue }
#subject:has(~ div > .test) { color: purple }
#subject:has(+ div .test) { color: yellow }
#subject:has(+ div > .test) { color: pink }
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
