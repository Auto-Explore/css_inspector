# css/css-shadow/host-in-host-context-selector.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/host-in-host-context-selector.html"
}
```

## style[0]

```css

      /* Should not match */
      :host-context(:host) {
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
