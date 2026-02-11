# css/css-contain/counter-scoping-004.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/counter-scoping-004.html"
}
```

## style[0]

```css

  .list-item {
    contain: style;
    counter-increment: my-list-counter;
    margin-left: 40px;
  }
  .list-item::before {
    content: '[' counter(my-list-counter, decimal) '] ';
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
