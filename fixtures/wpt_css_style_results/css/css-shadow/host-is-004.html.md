# css/css-shadow/host-is-004.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/host-is-004.html"
}
```

## style[0]

```css

      :host {
        width: 100px;
        height: 100px;
        background-color: green;
      }
      :is(:host(#not-host), #host) {
        background-color: red;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
