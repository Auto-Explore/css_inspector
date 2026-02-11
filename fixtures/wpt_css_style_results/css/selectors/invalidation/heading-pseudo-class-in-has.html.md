# css/selectors/invalidation/heading-pseudo-class-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/heading-pseudo-class-in-has.html"
}
```

## style[0]

```css

  div { color: grey }
  #ancestor:has(:heading(1)) ~ #has_heading_1 { color: red }
  #ancestor:has(:heading(2)) ~ #has_heading_2 { color: orange }
  #ancestor:has(:heading(3)) ~ #has_heading_3 { color: yellow }
  #ancestor:has(:heading(1, 2)) ~ #has_heading_1_2 { color: green }
  #sibling:has(+ :heading(1)) ~ #has_adjacent_heading_1 { color: blue }
  #sibling + :heading(1) ~ #adjacent_heading_1 { color: purple }
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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
