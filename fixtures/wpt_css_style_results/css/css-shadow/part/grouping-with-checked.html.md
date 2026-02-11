# css/css-shadow/part/grouping-with-checked.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/grouping-with-checked.html"
}
```

## style[0]

```css

  my-element::part(checkbox) {
    font-family: fantasy;
    background-color: #ff0000;
  }
  #grouped {
    color: #ff0000;
  }
  my-element::part(checkbox):checked {
    background-color: #00ff00;
  }
  my-element::part(checkbox):checked,
  #grouped {
    color: #00ff00;
  }
  my-element::part(not-a-part):checked,
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
