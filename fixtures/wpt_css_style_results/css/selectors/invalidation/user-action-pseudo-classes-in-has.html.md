# css/selectors/invalidation/user-action-pseudo-classes-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/user-action-pseudo-classes-in-has.html"
}
```

## style[0]

```css

  .ancestor:has(.descendant1:hover) { color: blue }
  .ancestor:has(.descendant1:hover) .other-descendant { color: navy }
  .ancestor:has(.descendant1:hover:active) { color: skyblue }
  .ancestor:has(.descendant1:hover:active) .other-descendant { color: lightblue }
  .ancestor:has(:focus) { color: green }
  .ancestor:has(:focus) .other-descendant { color: darkgreen }
  .ancestor:has(.descendant2:focus-visible) { color: yellowgreen }
  .ancestor:has(.descendant2:focus-visible) .other-descendant { color: greenyellow }
  .ancestor:has(.descendant3:focus-within) { color: lightgreen }
  .ancestor:has(.descendant3:focus-within) .other-descendant { color: violet }
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
