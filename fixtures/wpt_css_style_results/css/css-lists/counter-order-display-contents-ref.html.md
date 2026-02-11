# css/css-lists/counter-order-display-contents-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-order-display-contents-ref.html"
}
```

## style[0]

```css

      .counted {
        counter-increment: section;
      }

      .counted::before {
        content: "C=" counter(section) " ";
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
