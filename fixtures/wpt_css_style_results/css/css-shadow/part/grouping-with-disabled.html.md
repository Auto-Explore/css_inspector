# css/css-shadow/part/grouping-with-disabled.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/grouping-with-disabled.html"
}
```

## style[0]

```css

  my-element::part(button) {
    font-family: fantasy;
    background-color: #ff0000;
  }
  #grouped {
    color: #ff0000;
  }
  my-element::part(button):disabled {
    background-color: #00ff00;
  }
  my-element::part(button):disabled,
  #grouped {
    color: #00ff00;
  }
  my-element::part(not-a-part):disabled,
  #grouped {
    font-family: monospace;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
