# css/css-grid/animation/grid-template-rows-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/animation/grid-template-rows-001.html"
}
```

## style[0]

```css

    @keyframes anim {
      from {
        grid-template-rows: 20px 1fr;
      }
      to {
        grid-template-rows: 100px 1fr;
      }
    }
    .grid {
      display: grid;
      width: 400px;
      height: 400px;
      grid-gap: 10px;
      animation: anim 10s -5s paused linear;
    }
    span { border: 1px solid; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
