# css/css-ui/text-overflow-025.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/text-overflow-025.html"
}
```

## style[0]

```css


    tr::before {
      content: "Some long text here that overflows and whatnot.";
      display: table-cell;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
    tr {
      overflow: hidden;
    }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
