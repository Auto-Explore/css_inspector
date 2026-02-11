# css/css-values/getComputedStyle-border-radius-003.html

```json
{
  "format_version": 3,
  "file": "css/css-values/getComputedStyle-border-radius-003.html"
}
```

## style[0]

```css

  div#target
    {
      border: solid 2px;
      border-top-left-radius: calc(1px + 1%) calc(5px + 5%);
      border-top-right-radius: calc(2px + 2%) calc(6px + 6%);
      border-bottom-right-radius: calc(3px + 3%) calc(7px + 7%);
      border-bottom-left-radius: calc(4px + 4%) calc(8px + 8%);
      height: 100px;
      width: 100px;
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-top-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-left-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
