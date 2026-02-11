# css/css-shadow/host-in-host-selector.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/host-in-host-selector.html"
}
```

## style[0]

```css

      /* Should not match */
      :host(:host) {
        --x:FAIL;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
