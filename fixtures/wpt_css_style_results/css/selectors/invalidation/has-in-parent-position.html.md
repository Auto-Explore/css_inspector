# css/selectors/invalidation/has-in-parent-position.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-in-parent-position.html"
}
```

## style[0]

```css

div, main { color: grey }
div:has(.test) > #subject { color: red }
div:has([test_attr]) > #subject { color: orangered }
div:has(> .test) > #subject { color: green }
div:has(> [test_attr]) > #subject { color: lightgreen }
div:has(~ .test) >  #subject { color: yellow }
div:has(~ [test_attr]) >  #subject { color: ivory }
div:has(+ .test) > #subject { color: blue }
div:has(+ [test_attr]) > #subject { color: skyblue }
div:has(~ div .test) > #subject { color: purple }
div:has(~ div [test_attr]) > #subject { color: violet }
div:has(+ div .test) > #subject { color: pink }
div:has(+ div [test_attr]) > #subject { color: lightpink }
```

```json
{
  "errors": 8,
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
    },
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
