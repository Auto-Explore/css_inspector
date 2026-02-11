# css/css-writing-modes/wm-propagation-body-054.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-body-054.html"
}
```

## style[0]

```css

  html
    {
      writing-mode: sideways-lr;
    }

  html::after
    {
      content: "This text must be vertical, with letters upright.";
      text-orientation: upright;
      /* 'text-orientation: upright' has no effect with 'sideways-lr', but does with 'vertical-lr' */
      margin-top: 8px;
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
