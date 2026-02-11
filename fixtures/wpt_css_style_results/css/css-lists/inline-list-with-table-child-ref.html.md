# css/css-lists/inline-list-with-table-child-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/inline-list-with-table-child-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace;
}

.l1 { display: inline list-item; }
.l2 { display: inline flow-root list-item; }

span {
  border: 1px solid;
}

div { background: blue; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
