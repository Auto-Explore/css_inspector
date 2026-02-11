# css/css-shadow/host-functional-descendant-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/host-functional-descendant-invalidation.html"
}
```

## style[0]

```css

      :host ::slotted(div) { width: 100px; height: 100px; background: red; }
      :host(.foo) ::slotted(div) { background: green; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
