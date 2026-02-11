# css/css-lists/inline-block-list.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/inline-block-list.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:14px/1 monospace; padding:0; margin:0;
}
ol,ul,li { margin:0; padding:0; }
body { margin-left: 40px; }

li { border: 1px solid; display: inline flow-root list-item; }
.wrap { width: 22ch; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
