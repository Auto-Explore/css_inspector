# css/css-lists/counter-list-item.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counter-list-item.html"
}
```

## style[0]

```css

li:before {
    content: counter(list-item) " ";
}
li:after {
    content: " " counter(list-item);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
