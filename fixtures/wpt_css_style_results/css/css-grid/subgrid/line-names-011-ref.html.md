# css/css-grid/subgrid/line-names-011-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-011-ref.html"
}
```

## style[0]

```css

    html,body {
      color:black; background-color:white; font:12px/1 monospace; padding:0; margin:0;
    }

    div > div { background: grey; grid-column:2 / span 2; }

    i {
      grid-row: 1;
      counter-increment: i;
    }
    i::before { content: counter(i, decimal); }

    x { background: silver; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid-column”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
