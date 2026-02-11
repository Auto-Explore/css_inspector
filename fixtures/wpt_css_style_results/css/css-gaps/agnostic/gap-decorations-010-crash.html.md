# css/css-gaps/agnostic/gap-decorations-010-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/agnostic/gap-decorations-010-crash.html"
}
```

## style[0]

```css

    .crazy-class {
      height: 110px;
      width: 110px;
      display: flex;

      column-gap: 10px;

      column-rule-style: solid;
      column-rule-width: 1px; /* start at 1px */
    }

    .flex-item {
      width: 50px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
