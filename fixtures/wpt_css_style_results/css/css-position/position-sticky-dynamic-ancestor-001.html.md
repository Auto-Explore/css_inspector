# css/css-position/position-sticky-dynamic-ancestor-001.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-sticky-dynamic-ancestor-001.html"
}
```

## style[0]

```css

  body {
    margin: 0;
    display: flex;
  }

  #content {
    border: 10px dashed gray;
    height: 600vh;
  }

  #sidebar {
    align-self: start;
    background-color: white;
    border: 10px dashed gray;
    top: 0;
  }

  #sidebar-header {
    position: sticky;
    top: 0;
    background-color: rgba(255, 0, 0, .5);
  }

  #sidebar-content {
    padding: 5px;
    height: 200vh;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
