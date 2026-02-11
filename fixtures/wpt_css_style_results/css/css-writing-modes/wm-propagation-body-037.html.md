# css/css-writing-modes/wm-propagation-body-037.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-body-037.html"
}
```

## style[0]

```css

  html
    {
      writing-mode: horizontal-tb;
    }

  html::before
    {
      background-color: orange;
      content: "";
      display: block;
      height: 100px;
      margin-left: 8px;
      margin-top: 8px;
      margin-right: 1em;
      width: 100px;
    }

  body
    {
      writing-mode: vertical-lr;
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
