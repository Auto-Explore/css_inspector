# css/css-nesting/host-nesting-003.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/host-nesting-003.html"
}
```

## style[0]

```css

      .nested {
        width: 100px;
        height: 100px;
        background-color: green;
      }
      :host(#not-host), #host {
        .nested {
          background-color: red;
        }
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
