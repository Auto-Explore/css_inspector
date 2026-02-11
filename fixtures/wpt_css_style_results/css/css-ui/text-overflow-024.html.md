# css/css-ui/text-overflow-024.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/text-overflow-024.html"
}
```

## style[0]

```css


    tr::before {
      content: "Some long text here that overflows and whatnot.";
      display: table-cell;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
