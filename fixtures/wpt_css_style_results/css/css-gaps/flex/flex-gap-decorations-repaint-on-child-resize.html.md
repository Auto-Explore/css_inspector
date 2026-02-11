# css/css-gaps/flex/flex-gap-decorations-repaint-on-child-resize.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/flex/flex-gap-decorations-repaint-on-child-resize.html"
}
```

## style[0]

```css

    .flex-container {
      display: flex;
      column-gap: 20px;
      width: 160px;
      height: 50px;
      background: red;
      column-rule: 20px solid green;
    }

    .flex-item {
      background: green;
      height: 50px;
    }

    #item1 {
      /* Start at 40px, will change to 90px */
      width: 40px;
    }

    #item2 {
      width: 50px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
