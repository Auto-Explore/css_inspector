# css/css-ui/outline-017.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-017.html"
}
```

## style[0]

```css

@keyframes outline-anim {
  from {
    outline: solid 1px rgba(1, 0, 0, 0.5);
    outline-offset: 1px;
  }
  to {
    outline: solid 3px rgba(3, 0, 0, 0.5);
    outline-offset: 3px;
  }
}

#test {
  animation: outline-anim 3s -1.5s paused linear;
  outline: solid 1px rgba(1, 0, 0, 0.5);
  outline-offset: 1px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
