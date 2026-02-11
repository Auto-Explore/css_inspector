# css/css-content/attr-chained-pseudo-001.html

```json
{
  "format_version": 3,
  "file": "css/css-content/attr-chained-pseudo-001.html"
}
```

## style[0]

```css

      div {
        margin-left: 4em;
      }
      div::after {
        content: "World";
        display: list-item;
      }
      div::after::marker {
        content: attr(data-mark, "failed ");
        color: blue;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
