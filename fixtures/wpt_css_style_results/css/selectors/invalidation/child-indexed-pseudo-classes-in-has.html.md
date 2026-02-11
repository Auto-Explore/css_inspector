# css/selectors/invalidation/child-indexed-pseudo-classes-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/child-indexed-pseudo-classes-in-has.html"
}
```

## style[0]

```css

  #container ~ div { color: grey }
  #container:has(:only-child) ~ #only_child { color: red }
  #container:has(.orange:first-child) ~ #first_child { color: orange }
  #container:has(.yellow:first-child) ~ #first_child { color: yellow }
  #container:has(.green:first-child) ~ #first_child { color: green }
  #container:has(.orange:last-child) ~ #last_child { color: orange }
  #container:has(.yellow:last-child) ~ #last_child { color: yellow }
  #container:has(.green:last-child) ~ #last_child { color: green }
  #container:has(.orange:nth-child(3n)) ~ #nth_child_3n { color: orange }
  #container:has(.yellow:nth-child(3n)) ~ #nth_child_3n { color: yellow }
  #container:has(.green:nth-child(3n)) ~ #nth_child_3n { color: green }
  #container:has(.orange:nth-child(3n+1)) ~ #nth_child_3n_1 { color: orange }
  #container:has(.yellow:nth-child(3n+1)) ~ #nth_child_3n_1 { color: yellow }
  #container:has(.green:nth-child(3n+1)) ~ #nth_child_3n_1 { color: green }
  #container:has(.orange:nth-child(3n+2)) ~ #nth_child_3n_2 { color: orange }
  #container:has(.yellow:nth-child(3n+2)) ~ #nth_child_3n_2 { color: yellow }
  #container:has(.green:nth-child(3n+2)) ~ #nth_child_3n_2 { color: green }
  #container:has(.orange:nth-child(3n)) ~ #nth_child_3n { color: orange }
  #container:has(.yellow:nth-child(3n)) ~ #nth_child_3n { color: yellow }
  #container:has(.green:nth-child(3n)) ~ #nth_child_3n { color: green }
```

```json
{
  "errors": 7,
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
    }
  ],
  "warnings": 0
}
```
