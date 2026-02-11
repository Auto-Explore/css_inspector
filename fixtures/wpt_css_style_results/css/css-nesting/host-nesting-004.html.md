# css/css-nesting/host-nesting-004.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/host-nesting-004.html"
}
```

## style[0]

```css

      :host {
        width: 100px;
        height: 100px;
        background-color: green;
      }
      :host(#not-host), #host {
        @media (width >= 0) {
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
