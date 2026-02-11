# css/css-backgrounds/inheritance.sub.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/inheritance.sub.html"
}
```

## style[0]

```css

  #reference {
    column-rule-style: dotted; /* Avoid column-rule-width computed style 0px */
    column-rule-width: medium;
  }
  #container {
    border-style: solid; /* Avoid border-*-width computed style 0px */
  }
  #target {
    border-style: solid; /* Avoid border-*-width computed style 0px */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
