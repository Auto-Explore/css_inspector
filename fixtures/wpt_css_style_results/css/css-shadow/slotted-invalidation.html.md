# css/css-shadow/slotted-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/slotted-invalidation.html"
}
```

## style[0]

```css
.outer ::slotted(#slotted) { background-color: red } .outer .inner::slotted(#slotted) { background-color: green }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
