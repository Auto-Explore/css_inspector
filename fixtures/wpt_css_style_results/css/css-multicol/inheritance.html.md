# css/css-multicol/inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/inheritance.html"
}
```

## style[0]

```css

  #reference {
    border-style: dotted; /* Avoid border-top-width computed style 0 */
    border-top-width: medium;
  }
  #container {
    color: rgba(42, 53, 64, 0.75);
    column-rule-style: dotted; /* Avoid column-rule-width computed style 0 */
  }
  #target {
    column-rule-style: dotted;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
