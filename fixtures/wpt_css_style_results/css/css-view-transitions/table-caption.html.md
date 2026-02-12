# css/css-view-transitions/table-caption.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/table-caption.html"
}
```

## style[0]

```css

  table {
    view-transition-name: table;
  }

  :root::view-transition-group(root) {
    opacity: 0;
  }

  :root::view-transition-group(*) {
    animation-play-state: paused;
  }

  :root::view-transition {
    background: pink;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
