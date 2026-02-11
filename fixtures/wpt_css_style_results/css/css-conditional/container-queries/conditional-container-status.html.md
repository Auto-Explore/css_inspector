# css/css-conditional/container-queries/conditional-container-status.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/conditional-container-status.html"
}
```

## style[0]

```css

  .parent { width: 300px; }
  .child { width: 100px; }
  .parent, .child { container-type: inline-size; }
  @container (min-width: 200px) {
    .child { container-type: initial; }
    .grandchild { border: 3px solid green }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
