# css/css-writing-modes/wm-propagation-body-041.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-body-041.html"
}
```

## style[0]

```css

  html
    {
      writing-mode: vertical-rl;
    }

  body
    {
      writing-mode: vertical-lr;
    }

  div
    {
      background-color: orange;
      height: 100px;
      width: 100px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
